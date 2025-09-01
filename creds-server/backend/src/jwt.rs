use jsonwebtoken::{self, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    user: String,
    pub role: String,
}

impl Claims {
    #[allow(dead_code)]
    pub fn new(user: String, role: String) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let expiration_s = since_the_epoch.as_secs() + 60 * 60 * 24 * 100;

        Self {
            user, 
            role,
            exp: expiration_s as usize,
        }
    }
}

#[allow(dead_code)]
pub fn encode(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret("terrible secret".as_ref()),
    )
}

#[allow(dead_code)]
pub fn decode(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret("terrible secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}
