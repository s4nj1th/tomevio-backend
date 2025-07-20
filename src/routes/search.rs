
use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author_name: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn search_books(Query(params): Query<SearchQuery>) -> Json<Vec<Book>> {
    let query = params.q;
    let url = format!("https://openlibrary.org/search.json?q={}", query);

    let response = reqwest::get(&url).await.unwrap();
    let data: serde_json::Value = response.json().await.unwrap();

    let books: Vec<Book> = data["docs"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|doc| {
            Some(Book {
                title: doc["title"].as_str()?.to_string(),
                author_name: doc["author_name"]
                    .as_array()
                    .map(|authors| {
                        authors
                            .iter()
                            .filter_map(|a| a.as_str().map(|s| s.to_string()))
                            .collect()
                    }),
            })
        })
        .collect();

    Json(books)
}