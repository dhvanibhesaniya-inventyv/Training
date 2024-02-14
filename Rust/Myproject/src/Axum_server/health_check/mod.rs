use axum::{routing::get, Router};

pub fn get_status_routes() -> Router {
    Router::new().route("/health/check", get(root))
}

// basic handler that responds with a static string
async fn root() -> String {
    "Every Thing Is good.".to_string()
}
