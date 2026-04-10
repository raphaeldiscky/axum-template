use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID).
    pub sub: String,
    /// Email address.
    pub email: String,
    /// Expiration time (UTC timestamp).
    pub exp: usize,
    /// Issued at (UTC timestamp).
    pub iat: usize,
}

pub fn encode(secret: &str, claims: &Claims) -> Result<String, JwtError> {
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode(secret: &str, token: &str) -> Result<Claims, JwtError> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
