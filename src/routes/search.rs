use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author_name: Option<Vec<String>>,
    pub work_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub work_count: Option<u32>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub books: Vec<Book>,
    pub authors: Vec<Author>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn search(Query(params): Query<SearchQuery>) -> Json<SearchResult> {
    let query = params.q;

    // Fetch books
    let books_url = format!("https://openlibrary.org/search.json?q={}", query);
    let books_response = reqwest::get(&books_url).await.unwrap();
    let books_data: serde_json::Value = books_response.json().await.unwrap();

    let books: Vec<Book> = books_data["docs"]
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
                work_id: doc["key"]
                    .as_str()
                    .and_then(|key| key.strip_prefix("/works/").map(|s| s.to_string())),
            })
        })
        .collect();

    // Fetch authors
    let authors_url = format!("https://openlibrary.org/search/authors.json?q={}", query);
    let authors_response = reqwest::get(&authors_url).await.unwrap();
    let authors_data: serde_json::Value = authors_response.json().await.unwrap();

    let authors: Vec<Author> = authors_data["docs"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|doc| {
            Some(Author {
                name: doc["name"].as_str()?.to_string(),
                work_count: doc["work_count"].as_u64().map(|wc| wc as u32),
            })
        })
        .collect();

    Json(SearchResult { books, authors })
}