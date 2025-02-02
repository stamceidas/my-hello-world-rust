use axum::{routing::get, Router, response::Json};
use serde_json::json;

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!("Hello, world from Shuttle! 🚀 "))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
