use axum::{routing::get, Router};
mod routes;

#[tokio::main]
async fn main() {
    let addr = "localhost";
    let port = "8080";

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(routes::search::search_books));
    println!("Server running on http://{}:{}", addr, port);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}