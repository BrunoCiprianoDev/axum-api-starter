use axum::{Router, response::Json, routing::get};

use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Message {
    status: String,
    message: String,
}

async fn get_user() -> Json<Message> {
    let response = Message {
        status: "Success".to_string(),
        message: "JSON RETURN".to_string(),
    };
    Json(response)
}

async fn root() -> &'static str {
    "First RUST API"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
