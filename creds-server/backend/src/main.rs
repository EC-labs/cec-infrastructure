use anyhow::Result;
use jwt::Claims;
use r2d2::Pool;
use r2d2_sqlite::{rusqlite::params, SqliteConnectionManager};
use serde_json::{json, Value};
use tokio;
use tracing::{info, Level};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use axum::{
    extract::{Query, State}, http::StatusCode, routing::{get, post}, Json, Router
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

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
            INSERT INTO user(email, role) VALUES (?, ?)
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
    
    let claims = Claims::new(email, String::from("student"));
    let student_token = jwt::encode(&claims).unwrap();
    
    Ok((StatusCode::CREATED, Json(user)))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let manager = SqliteConnectionManager::file("db.sqlite");
    let pool = r2d2::Pool::new(manager).unwrap();
    sql::init_sql(pool.clone())?;

    let app = Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_users))
        .route("/api/user", get(get_user))
        .route("/api/authenticate", get(authenticate))
        .with_state(pool);

    let bind_address = "[::1]:3000";
    info!("Server listening on {bind_address}");

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap(); 

    Ok(())
}
