use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub description: Option<String>,
    pub author_keys: Vec<String>,
}

#[derive(Deserialize)]
struct WorkResponse {
    title: String,
    #[serde(default)]
    description: Option<DescriptionField>,
    #[serde(default)]
    authors: Vec<AuthorEntry>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DescriptionField {
    String(String),
    Object { value: String },
}

#[derive(Deserialize)]
struct AuthorEntry {
    author: AuthorKey,
}

#[derive(Deserialize)]
struct AuthorKey {
    key: String,
}

pub async fn get_books(Path(work_id): Path<String>) -> Json<Book> {
    let url = format!("https://openlibrary.org/works/{}.json", work_id);

    let response = reqwest::get(&url)
        .await
        .expect("Failed to make request to OpenLibrary");
    let data: WorkResponse = response
        .json()
        .await
        .expect("Failed to parse OpenLibrary response");

    // Normalize description
    let description = match data.description {
        Some(DescriptionField::String(s)) => Some(s),
        Some(DescriptionField::Object { value }) => Some(value),
        None => None,
    };

    // Extract just the author keys
    let author_keys = data
        .authors
        .into_iter()
        .map(|entry| entry.author.key)
        .collect();

    let book = Book {
        title: data.title,
        description,
        author_keys,
    };

    Json(book)
}
