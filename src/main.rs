
mod handler;
mod model;
mod route;
mod schema;

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method
};

use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

use axum::{response::IntoResponse, routing::get, Json, Router};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub struct AppState {
    db: Pool<Postgres>
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    println!("{:?}", std::env::var("DATABASE_URL"));
    let database_url = String::from("postgres://postgres:pass123@localhost/livecode");
    // std::env::var("DATABASE_URL").expect("DATABASE URL Must be Set");
    //String::from("postgres://postgres:pass123@localhost/livecode"); // 

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
// ("http://localhost:3000".parse::<HeaderValue>().unwrap())
    let wildcard_origin = HeaderValue::from_str("*").unwrap();

    let cors = CorsLayer::new()
        .allow_origin(wildcard_origin)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(false)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
        

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);


    println!("Server Started Successfully");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

