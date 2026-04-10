use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID).
    pub sub: Uuid,
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

    fn test_user_id() -> Uuid {
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("valid uuid")
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn sample_claims() -> Claims {
        let now = chrono::Utc::now();
        Claims {
            sub: test_user_id(),
            email: "test@example.com".to_string(),
            exp: (now + chrono::Duration::hours(1)).timestamp() as usize,
            iat: now.timestamp() as usize,
        }
    }

    #[test]
    fn encode_and_decode_roundtrip() {
        let secret = "test-secret";
        let claims = sample_claims();

        let token = encode(secret, &claims).expect("failed to encode");
        let decoded = decode(secret, &token).expect("failed to decode");

        assert_eq!(decoded.sub, test_user_id());
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
            sub: test_user_id(),
            email: "test@example.com".to_string(),
            exp: 0,
            iat: 0,
        };

        let token = encode(secret, &claims).expect("failed to encode");
        let result = decode(secret, &token);

        assert!(result.is_err());
    }
}
