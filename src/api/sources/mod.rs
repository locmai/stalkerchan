use crate::api::{Error, Result};
use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceBody<T = Source> {
    source: T,
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub active: bool,
    pub target_type: String,
    pub group_id: String,
}

#[derive(serde::Serialize)]
struct TagsBody {
    tags: Vec<String>,
}

async fn get_source(
    // maybe_auth_user: MaybeAuthUser,
    // ctx: Extension<ApiContext>,
    Path(group_id): Path<String>,
) -> Result<Json<SourceBody>> {
    // mock database
    let sources = vec![
        Source {
            active: true,
            target_type: "GitHub".to_string(),
            group_id: "abc".to_string(),
        },
        Source {
            active: true,
            target_type: "Website".to_string(),
            group_id: "def".to_string(),
        },
    ];

    for source in sources.iter() {
        if source.group_id == group_id {
            tracing::debug!("Found group_id {}", group_id);
            return Ok(Json(SourceBody {
                source: source.clone(),
            }));
        }
    }

    tracing::debug!("Target source not found with group_id {}", group_id);
    Err(Error::NotFound)
}

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/test", get(|| async {}))
        .route("/api/sources/:group_id", get(get_source))
    //.route("/api/sources", get(list_sources))
    //.route("/api/sources", post(create_source).get(list_sources))
}
