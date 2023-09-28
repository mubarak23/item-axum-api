use std::sync::Arc;

use dotenv::dotenv;
use axum::{response::IntoResponse, routing::get, Json, Router};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Item Restful API with SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "Success",
        "message": MESSAGE
    });
    Json(json_response)
}

pub struct AppState {
    db: Pool<Postgres>
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL Must be Set");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url).
        await
     {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new().route("/api/healthcheck", get(health_checker_handler));

    println!("Server Started Successfully");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// run rust program on watch  use => cargo watch -q -c -w src/ -x run

// RUST_BACKTRACE=1 cargo run
// RUST_BACKTRACE=1 cargo watch -q -c -w src/ -x run

