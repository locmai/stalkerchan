use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use stalkerchan::core;

use config::{Config, File};
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() {

    let settings = Config::builder()
        .add_source(File::from(Path::new("config.yaml")))
        .build()
        .unwrap().try_deserialize::<HashMap<String, String>>().expect("Invalid configuration");
    
        // TODO: make a parser instead
    println!("{}", settings.get("rust_log").unwrap());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| settings.get("rust_log").unwrap().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = core::default_router().layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
