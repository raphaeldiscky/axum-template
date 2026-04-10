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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_claims() -> Claims {
        let now = chrono::Utc::now();
        let exp = usize::try_from((now + chrono::Duration::hours(1)).timestamp())
            .expect("timestamp must be positive");
        let iat = usize::try_from(now.timestamp()).expect("timestamp must be positive");
        Claims {
            sub: "user-123".to_string(),
            email: "test@example.com".to_string(),
            exp,
            iat,
        }
    }

    #[test]
    fn encode_and_decode_roundtrip() {
        let secret = "test-secret";
        let claims = sample_claims();

        let token = encode(secret, &claims).expect("failed to encode");
        let decoded = decode(secret, &token).expect("failed to decode");

        assert_eq!(decoded.sub, "user-123");
        assert_eq!(decoded.email, "test@example.com");
    }

    #[test]
    fn decode_with_wrong_secret_fails() {
        let claims = sample_claims();
        let token = encode("correct-secret", &claims).expect("failed to encode");
        let result = decode("wrong-secret", &token);

        assert!(result.is_err());
    }

    #[test]
    fn decode_expired_token_fails() {
        let secret = "test-secret";
        let claims = Claims {
            sub: "user-123".to_string(),
            email: "test@example.com".to_string(),
            exp: 0, // expired
            iat: 0,
        };

        let token = encode(secret, &claims).expect("failed to encode");
        let result = decode(secret, &token);

        assert!(result.is_err());
    }
}
