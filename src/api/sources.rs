use axum::{routing::get, Router};

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new().route("/api/test", get(|| async {}))
    // .route("/api/sources", get(list_sources))
    //.route("/api/sources", post(create_source).get(list_sources))
    // .route("/api/sources/:slug", get(get_source).put(update_source).delete(delete_source))
}
