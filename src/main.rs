use axum::{routing::{get, post}, Router};
use sea_orm::Database;
use std::net::SocketAddr;

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let db = Database::connect(conn_str)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(routes::search::search))
        .route("/book/{id}", get(routes::book::get_book))
        .route("/author/{id}", get(routes::author::get_author));

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
