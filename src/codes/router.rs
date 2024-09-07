use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/", get(|| async { "서버 되었냐?" }))
}
