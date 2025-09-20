use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use dotenvy::dotenv;

use crate::controller::{get_info_handler, login_handler};

mod routes;
mod controller;
mod model;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr_str = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let addr: SocketAddr = addr_str.parse().expect("Invalid SERVER_ADDR format");
    // let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    // let db = Database::connect(conn_str)
    //     .await
    //     .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(routes::search::search))
        .route("/book/{id}", get(routes::book::get_book))
        .route("/author/{id}", get(routes::author::get_author))
        .route("/login", post(login_handler))
        .route("/profile", get(get_info_handler));

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
