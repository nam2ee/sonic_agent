use std::sync::Arc;
use sonic_defai_ai::deepseek::DeepSeek;
use crate::types::AppState;
use axum::{http, Router};
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::recommend;

//WARNING!! Change CORS!
pub async fn build_server() {
    let shared_state = AppState::<DeepSeek>::new().await;

    let shared_state = Arc::new(shared_state);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS
        ])
        .allow_headers(Any)
        .allow_credentials(false);

    let app = Router::new().route("/recommend", get(recommend))
        .with_state(shared_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:443").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}