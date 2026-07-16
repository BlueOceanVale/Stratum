use crate::models::models::Claims;
use chrono::{Utc, Duration};
use jsonwebtoken;
use crate::models::models::User;

pub fn create_token(user: &User) -> Result<String, jsonwebtoken::error::Error> {
    let now = Utc::now();
    let expiration = now + Duration::hours(1);
    let secret = std::env::var("JWT_SECRET").unwrap();

    let claims = Claims {
        sub: user.id,
        email: user.email.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    let header = jsonwebtoken::Header::default();

    let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

    jsonwebtoken::encode(&header, &claims, &encoding_key)
}

pub fn verify_token(token: &str) -> Result<Claims, Error> {
    let secret = std::env::var("JWT_SECRET").unwrap();
    
}

