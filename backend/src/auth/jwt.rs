use crate::models::models::Claims;
use chrono::{Utc, Duration};
use jsonwebtoken::{Algorithm, Validation, decode, encode, Header, DecodingKey, EncodingKey, errors::Error};
use crate::models::models::User;

fn jwt_secret() -> String {
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    let _ = dotenvy::from_path(&env_path);
    dotenvy::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-jwt-secret-change-me".to_string())
}

pub fn create_token(user: &User) -> Result<String, Error> {
    let now = Utc::now();
    let expiration = now + Duration::hours(1);
    let secret = jwt_secret();

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
    let secret = jwt_secret();
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn creates_and_verifies_jwt() {
        let user = User {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
        };

        let token = create_token(&user).expect("token creation should work");
        let claims = verify_token(&token).expect("token verification should work");

        assert_eq!(claims.email, user.email);
        assert_eq!(claims.sub, user.id);
    }
}