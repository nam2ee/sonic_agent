use std::sync::Arc;
use crate::types::AppState;
use axum::{http, Router};
use axum::routing::post;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use sonic_defai_ai::claude::Claude;
use crate::handlers::recommend;

//WARNING!! Change CORS!
pub async fn build_server() {
    let shared_state = AppState::<Claude>::new().await;
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

    let api_routers = Router::new().route("/recommend", post(recommend))
        .with_state(shared_state)
        .layer(cors);

    let static_service = ServeDir::new("static");

    let app = Router::new()
        .nest("/api", api_routers)
        .fallback_service(static_service);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}