use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get_service, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::ServeDir;

use crate::AppState;

mod auth;
mod media;

pub fn build(state: AppState) -> Router {
    Router::new()
        .nest_service(
            "/media",
            get_service(ServeDir::new(&state.config.file.root)),
        )
        .route("/upload", post(media::upload))
        .route("/delete", delete(media::delete))
        .route("/new-otp", post(auth::new_otp))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(255 * 1024 * 1024))
        .with_state(state)
}
