use axum::{routing::get, Router};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(routes::search::search))
        .route("/book/{id}", get(routes::book::get_books))
        .route("/author/{id}", get(routes::author::get_author));

    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
