use axum::Router;

mod sources;

pub fn api_router() -> Router {
    sources::router()
}
