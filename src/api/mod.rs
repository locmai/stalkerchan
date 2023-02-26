use axum::Router;

mod sources;

/// Defines a common error type to use for all request handlers
mod error;
pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn api_router() -> Router {
    sources::router()
}
