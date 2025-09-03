use indoc::{formatdoc, indoc};
use anyhow::Result;
use jwt::Claims;
use r2d2::Pool;
use r2d2_sqlite::{rusqlite::params, SqliteConnectionManager};
use serde_json::{json, Value};
use tokio;
use tracing::{info, Level};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use axum::{
    body::Body, extract::{Query, RawPathParams, State}, http::{header, StatusCode}, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use axum_macros::debug_handler;
use std::{collections::HashMap, env, fs::{self, File}, path::Path, sync::Arc};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;

use sendgrid::error::SendgridError;
use sendgrid::v3::*;

mod jwt;
mod sql;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
}

#[derive(Serialize, Debug)]
struct User {
    email: String,
    client: u64,
    group: Option<u64>,
    role: String,
}

#[derive(Deserialize)]
struct Token {
    token: String,
}


async fn authenticate(jar: CookieJar, query: Query<Token>) -> Result<(CookieJar, StatusCode), StatusCode> {
    match jwt::decode(&query.token) {
        Ok(_) => {
            Ok((
                jar.add(
                    Cookie::build(("token", query.token.clone()))
                        .http_only(true)
                )
                ,
                StatusCode::OK,
            ))
        },
        Err(_) => {
            info!("invalid token `{}`", query.token);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

type Users = Vec<User>;

async fn get_users(
    State(pool): State<Pool<SqliteConnectionManager>>,
    jar: CookieJar,
) -> Result<(StatusCode, Json<Users>), StatusCode> {
    let token = jar.get("token").ok_or(StatusCode::NOT_FOUND)?;
    let email = match jwt::decode(token.value()) {
        Ok(token_data) => {
            let role = token_data.claims.role;
            if role != "admin" {
                info!("Attempting to create user with role {role}");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }, 
        Err(_) => {
            return Err(StatusCode::FORBIDDEN);
        }
    };

    let conn = pool.get().unwrap(); 
    let mut query = conn
        .prepare("
            SELECT email, client_id, group_id, role FROM user;
        ")
        .unwrap();
    let users: Vec<User> = query.query_map([], |row| {
            Ok(User {
                email: row.get::<usize, String>(0).unwrap(),
                client: row.get::<usize, u64>(1).unwrap(),
                group: row.get::<usize, Option<u64>>(2).unwrap(),
                role: row.get::<usize, String>(3).unwrap(),
            })
        })
        .unwrap()
        .map(|user| user.unwrap())
        .collect();

    return Ok((StatusCode::OK, Json(users)))
}

async fn get_user(
    State(pool): State<Pool<SqliteConnectionManager>>,
    jar: CookieJar,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let token = jar.get("token").ok_or(StatusCode::NOT_FOUND)?;
    let email = match jwt::decode(token.value()) {
        Ok(token_data) => {
            token_data.claims.user
        }, 
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let conn = pool.get().unwrap(); 
    let user = conn
        .prepare("
            SELECT email, client_id, group_id, role FROM user WHERE email = ?;
        ").unwrap()
        .query_row(params![email], |row| {
            Ok(User {
                email: row.get::<usize, String>(0).unwrap(),
                client: row.get::<usize, u64>(1).unwrap(),
                group: row.get::<usize, Option<u64>>(2).unwrap(),
                role: row.get::<usize, String>(3).unwrap(),
            })
        }).expect(&format!("User `{}` missing from db", email));

    return Ok((StatusCode::OK, Json(user)))
}

type Files = Vec<String>;

async fn get_files(
    State(pool): State<Pool<SqliteConnectionManager>>,
    jar: CookieJar,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let token = jar.get("token").ok_or(StatusCode::NOT_FOUND)?;
    let email = match jwt::decode(token.value()) {
        Ok(token_data) => {
            token_data.claims.user
        }, 
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let conn = pool.get().unwrap(); 
    let user = conn
        .prepare("
            SELECT email, client_id, group_id, role FROM user WHERE email = ?;
        ").unwrap()
        .query_row(params![email], |row| {
            Ok(User {
                email: row.get::<usize, String>(0).unwrap(),
                client: row.get::<usize, u64>(1).unwrap(),
                group: row.get::<usize, Option<u64>>(2).unwrap(),
                role: row.get::<usize, String>(3).unwrap(),
            })
        }).expect(&format!("User `{}` missing from db", email));

    let credentials_dir = env::var("CREDENTIALS_DIR").expect("CREDENTIALS_DIR unset");

    let mut body = serde_json::Map::new();
    let client = format!("client{}", user.client);
    let client_dir = Path::new(&credentials_dir).join("clients").join(&client);
    let mut client_files = Vec::new();
    for entry in fs::read_dir(client_dir).unwrap().into_iter().map(|entry| entry.unwrap()) {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        client_files.push(file_name.into());
    }
    body.insert(client, Value::Array(client_files));

    let Some(group) = user.group else {
        return Ok((StatusCode::OK, Json(Value::Object(body))));
    };
    let group = format!("group{}", group);
    let group_dir = Path::new(&credentials_dir).join("groups").join(&group);
    let mut group_files = Vec::new();
    for entry in fs::read_dir(group_dir).unwrap().into_iter().map(|entry| entry.unwrap()) {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        group_files.push(file_name.into());
    }
    body.insert(group, Value::Array(group_files));
    Ok((StatusCode::OK, Json(Value::Object(body))))
}

async fn sendgrid_email(to_email: &str, token: &str) -> Result<()> {
    let p = Personalization::new(Email::new(to_email));
    info!("send email to {to_email}");

    let content = formatdoc!(r#"
        visit <a href="https://cec-creds.ad.dlandau.nl/login">https://cec-creds.ad.dlandau.nl/login</a> and paste the following token: 
        {}
    "#, token);
    let m = Message::new(Email::new("noreply-infomcec@dlandau.nl"))
        .set_subject("[INFOMCEC] Credentials")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(content),
        )
        .add_personalization(p);

    let mut api_key = env::var("SG_API_KEY").expect("Missing SG_API_KEY");
    let sender = Sender::new(api_key, None);
    sender.send(&m).await?;
    
    Ok(())
}

#[debug_handler]
async fn send_email(
    State(pool): State<Pool<SqliteConnectionManager>>,
    jar: CookieJar,
    params: RawPathParams,
) -> Result<StatusCode, StatusCode> {
    let token = jar.get("token").ok_or(StatusCode::UNAUTHORIZED)?;
    match jwt::decode(token.value()) {
        Ok(token_data) => {
            let role = token_data.claims.role;
            if role != "admin" {
                info!("Attempting to create user with role {role}");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }, 
        Err(_) => {
            return Err(StatusCode::FORBIDDEN);
        }
    };

    let user_email = match params.iter().filter(|(key, _)| *key == "email").map(|(_, v)| v).next() {
        Some(email) => email,
        None => return Err(StatusCode::NOT_FOUND)
    };
    let conn = pool.get().unwrap(); 
    let user = conn
        .prepare("
            SELECT email, client_id, group_id, role FROM user WHERE email = ?;
        ").unwrap()
        .query_row(params![user_email], |row| {
            Ok(User {
                email: row.get::<usize, String>(0).unwrap(),
                client: row.get::<usize, u64>(1).unwrap(),
                group: row.get::<usize, Option<u64>>(2).unwrap(),
                role: row.get::<usize, String>(3).unwrap(),
            })
        }).expect(&format!("User `{}` missing from db", user_email));
    
    let claims = Claims::new(user.email.clone(), user.role);
    let student_token = jwt::encode(&claims).unwrap();

    if let Err(err) = sendgrid_email(&user.email, &student_token).await {
        eprintln!("Sendgrid error: {:?}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(StatusCode::OK)
}

async fn create_user(
    State(pool): State<Pool<SqliteConnectionManager>>,
    jar: CookieJar,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let token = jar.get("token").ok_or(StatusCode::UNAUTHORIZED)?;
    let email = match jwt::decode(token.value()) {
        Ok(token_data) => {
            let role = token_data.claims.role;
            if role != "admin" {
                info!("Attempting to create user with role {role}");
                return Err(StatusCode::UNAUTHORIZED);
            }
            payload.email
        }, 
        Err(_) => {
            return Err(StatusCode::FORBIDDEN);
        }
    };

    let conn = pool.get().unwrap(); 
    let user = conn
        .prepare("
            INSERT INTO user(client_id, email, role) 
            SELECT MAX(client_id) + 1, ?, ?
            FROM user
            RETURNING email, client_id, group_id, role
        ").unwrap()
        .query_row(params![email, "student"], |row| {
            Ok(User {
                email: row.get::<usize, String>(0).unwrap(),
                client: row.get::<usize, u64>(1).unwrap(),
                group: row.get::<usize, Option<u64>>(2).unwrap(),
                role: row.get::<usize, String>(3).unwrap(),
            })
        })
        .map_err(|e| StatusCode::FORBIDDEN)?;
    
    Ok((StatusCode::CREATED, Json(user)))
}

async fn download_file(
    State(pool): State<Pool<SqliteConnectionManager>>,
    jar: CookieJar, 
    params: RawPathParams
) -> impl IntoResponse {
    let requested_file = match params.iter().filter(|(key, _)| *key == "file").map(|(_, v)| v).next() {
        Some(file) => file,
        None => return Err(StatusCode::NOT_FOUND)
    };

    let token = jar.get("token").ok_or(StatusCode::NOT_FOUND)?;
    let email = match jwt::decode(token.value()) {
        Ok(token_data) => {
            token_data.claims.user
        }, 
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let conn = pool.get().unwrap(); 
    let user = conn
        .prepare("
            SELECT email, client_id, group_id, role FROM user WHERE email = ?;
        ").unwrap()
        .query_row(params![email], |row| {
            Ok(User {
                email: row.get::<usize, String>(0).unwrap(),
                client: row.get::<usize, u64>(1).unwrap(),
                group: row.get::<usize, Option<u64>>(2).unwrap(),
                role: row.get::<usize, String>(3).unwrap(),
            })
        }).expect(&format!("User `{}` missing from db", email));

    let conn = pool.get().unwrap(); 

    let mut valid_files = vec![format!("client{}.zip", user.client)];
    if let Some(group) = user.group {
        valid_files.push(format!("group{}.zip", group))
    }

    if !valid_files.iter().any(|v| v==requested_file) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let credentials_dir = env::var("CREDENTIALS_DIR").expect("CREDENTIALS_DIR unset");
    let mut file_path = Path::new(&credentials_dir).to_path_buf();
    if requested_file.starts_with("client") {
        file_path.push("clients")
    } else {
        file_path.push("groups")
    };
    file_path.push(requested_file);

    let file = match tokio::fs::File::open(file_path).await {
        Ok(file) => file,
        Err(err) => return Err(StatusCode::NOT_FOUND),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, String::from("application/zip")),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", requested_file),
        ),
    ];

    Ok((headers, body))
}

async fn send_admin_token() -> Result<()> {
    let token_check = Path::new("./data/token_sent");
    if token_check.exists() {
        return Ok(())
    }

    info!("send admin token");
    let claims = Claims::new(String::from("d.landau@uu.nl"), String::from("admin"));
    let admin_token = jwt::encode(&claims)?;
    sendgrid_email(&claims.user, &admin_token).await?;
    fs::create_dir_all(token_check.parent().unwrap());
    File::create(token_check);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let db = Path::new("./data/db.sqlite");
    fs::create_dir_all(db.parent().unwrap());
    let manager = SqliteConnectionManager::file(db);
    let pool = r2d2::Pool::new(manager).unwrap();
    sql::init_sql(pool.clone())?;
    send_admin_token().await;

    let app = Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_users))
        .route("/api/user", get(get_user))
        .route("/api/user/{email}/send_email", post(send_email))
        .route("/api/authenticate", get(authenticate))
        .route("/api/files", get(get_files))
        .route("/api/download/{file}", get(download_file))
        .with_state(pool);

    let bind_address = "[::1]:3000";
    info!("Server listening on {bind_address}");

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap(); 

    Ok(())
}
