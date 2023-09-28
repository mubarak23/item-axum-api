use axum::{response::IntoResponse, routing::get, Json, Router};

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Item Restful API with SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "Success",
        "message": MESSAGE
    });
    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthcheck", get(health_checker_handler));

    println!("Server Started Successfully");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// run rust program on watch  use => cargo watch -q -c -w src/ -x run