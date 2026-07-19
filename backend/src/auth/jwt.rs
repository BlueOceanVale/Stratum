use crate::models::models::Claims;
use chrono::{Utc, Duration};
use jsonwebtoken::{Algorithm, Validation, decode, encode, Header, DecodingKey, EncodingKey, errors::Error};
use crate::models::models::User;

pub fn create_token(user: &User) -> Result<String, Error> {
    let now = Utc::now();
    let expiration = now + Duration::hours(1);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        sub: user.id,
        email: user.email.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    let header = Header::default();

    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    encode(&header, &claims, &encoding_key)
}

pub fn verify_token(token: &str) -> Result<Claims, Error> {
    let validation = Validation::new(Algorithm::HS256);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    
    Ok(token_data.claims)
}