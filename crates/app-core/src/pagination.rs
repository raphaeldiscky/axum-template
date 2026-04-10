use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_LIMIT: i64 = 10;
const MIN_LIMIT: i64 = 1;
const MAX_LIMIT: i64 = 100;

/// Query parameters for cursor-based pagination.
///
/// Extract from requests via `Query<CursorParams>`.
#[derive(Debug, Clone, Deserialize)]
pub struct CursorParams {
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub cursor: Option<String>,
}

const fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

impl Default for CursorParams {
    fn default() -> Self {
        Self {
            limit: DEFAULT_LIMIT,
            cursor: None,
        }
    }
}

impl CursorParams {
    #[must_use]
    pub fn clamp(mut self) -> Self {
        self.limit = self.limit.clamp(MIN_LIMIT, MAX_LIMIT);
        self
    }
}

/// Encoded cursor data for keyset pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorData {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl CursorData {
    pub fn encode(&self) -> String {
        use base64::Engine;
        let json = serde_json::to_string(self).unwrap_or_default();
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json.as_bytes())
    }

    pub fn decode(cursor: &str) -> Option<Self> {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(cursor)
            .ok()?;
        let json = String::from_utf8(bytes).ok()?;
        serde_json::from_str(&json).ok()
    }
}

/// Cursor-based pagination metadata included in list responses.
#[derive(Debug, Clone, Serialize)]
pub struct CursorPagination {
    pub next_cursor: Option<String>,
    pub has_next: bool,
    pub limit: i64,
}

impl CursorPagination {
    /// Build pagination from query results.
    ///
    /// The query should fetch `limit + 1` rows. If more rows than `limit`
    /// are returned, the extra row is removed and `has_next` is set to true.
    pub fn from_results<T, F>(items: &mut Vec<T>, limit: i64, cursor_fn: F) -> Self
    where
        F: Fn(&T) -> CursorData,
    {
        let limit_usize = usize::try_from(limit).unwrap_or(usize::MAX);
        let has_next = items.len() > limit_usize;
        if has_next {
            items.truncate(limit_usize);
        }
        let next_cursor = if has_next {
            items.last().map(|item| cursor_fn(item).encode())
        } else {
            None
        };
        Self {
            next_cursor,
            has_next,
            limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_encode_decode_roundtrip() {
        let data = CursorData {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("valid uuid"),
            created_at: Utc::now(),
        };
        let encoded = data.encode();
        let decoded = CursorData::decode(&encoded).expect("decode failed");
        assert_eq!(decoded.id, data.id);
    }

    #[test]
    fn cursor_decode_invalid_returns_none() {
        assert!(CursorData::decode("not-valid-base64!@#").is_none());
        assert!(CursorData::decode("").is_none());
    }

    #[test]
    fn from_results_with_more_items() {
        let mut items: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let pagination = CursorPagination::from_results(&mut items, 5, |_| CursorData {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
        });
        assert_eq!(items.len(), 5);
        assert!(pagination.has_next);
        assert!(pagination.next_cursor.is_some());
    }

    #[test]
    fn from_results_without_more_items() {
        let mut items: Vec<i32> = vec![1, 2, 3];
        let pagination = CursorPagination::from_results(&mut items, 5, |_| CursorData {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
        });
        assert_eq!(items.len(), 3);
        assert!(!pagination.has_next);
        assert!(pagination.next_cursor.is_none());
    }

    #[test]
    fn clamp_enforces_bounds() {
        let params = CursorParams {
            limit: 500,
            cursor: None,
        }
        .clamp();
        assert_eq!(params.limit, MAX_LIMIT);

        let params = CursorParams {
            limit: -5,
            cursor: None,
        }
        .clamp();
        assert_eq!(params.limit, MIN_LIMIT);
    }
}
