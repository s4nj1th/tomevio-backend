use axum::{routing::{get, post}, Router};
use sea_orm::Database;
use std::net::SocketAddr;
use dotenvy::dotenv;

mod routes;
mod auth;
mod entity;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // let conn_str = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    // let db = Database::connect(conn_str)
    //     .await
    //     .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(routes::search::search))
        .route("/book/{id}", get(routes::book::get_book))
        .route("/author/{id}", get(routes::author::get_author));
        // .route("/signin", post(routes::signin::signin))
        // .route("/signup", post(routes::signup::signup));

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
