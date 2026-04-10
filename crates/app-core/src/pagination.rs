use serde::{Deserialize, Serialize};

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_SIZE: i64 = 10;
const MIN_PAGE: i64 = 1;
const MAX_PAGE: i64 = 1000;
const MIN_SIZE: i64 = 1;
const MAX_SIZE: i64 = 100;

/// Query parameters for offset-based pagination.
///
/// Extract from requests via `Query<PaginationParams>`.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_size")]
    pub size: i64,
}

const fn default_page() -> i64 {
    DEFAULT_PAGE
}

const fn default_size() -> i64 {
    DEFAULT_SIZE
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            size: DEFAULT_SIZE,
        }
    }
}

impl PaginationParams {
    /// Clamp page and size to valid bounds.
    #[must_use]
    pub fn clamp(mut self) -> Self {
        self.page = self.page.clamp(MIN_PAGE, MAX_PAGE);
        self.size = self.size.clamp(MIN_SIZE, MAX_SIZE);
        self
    }

    /// Calculate the database offset from page and size.
    pub const fn offset(&self) -> i64 {
        (self.page - 1) * self.size
    }
}

/// Offset-based pagination metadata included in list responses.
#[derive(Debug, Clone, Serialize)]
pub struct OffsetPagination {
    pub page: i64,
    pub size: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

impl OffsetPagination {
    pub const fn new(total_items: i64, page: i64, size: i64) -> Self {
        let total_pages = if size > 0 {
            (total_items + size - 1) / size
        } else {
            0
        };
        Self {
            page,
            size,
            total_items,
            total_pages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_calculation() {
        let params = PaginationParams { page: 1, size: 10 };
        assert_eq!(params.offset(), 0);

        let params = PaginationParams { page: 3, size: 20 };
        assert_eq!(params.offset(), 40);
    }

    #[test]
    fn clamp_enforces_bounds() {
        let params = PaginationParams {
            page: -5,
            size: 500,
        }
        .clamp();
        assert_eq!(params.page, MIN_PAGE);
        assert_eq!(params.size, MAX_SIZE);
    }

    #[test]
    fn pagination_metadata() {
        let meta = OffsetPagination::new(95, 2, 10);
        assert_eq!(meta.total_pages, 10);
        assert_eq!(meta.total_items, 95);

        let meta = OffsetPagination::new(0, 1, 10);
        assert_eq!(meta.total_pages, 0);
    }
}
