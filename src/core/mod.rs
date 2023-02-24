use axum::{middleware, Router};
mod metrics;

use crate::api::api_router;

pub fn default_router() -> Router {
    metrics::metrics_router()
        .merge(api_router())
        .route_layer(middleware::from_fn(metrics::track_metrics))
}
