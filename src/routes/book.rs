use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub description: Option<String>,
    pub authors: Vec<Author>,
    pub covers: Option<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub key: String,
}

#[derive(Deserialize)]
struct WorkResponse {
    title: String,
    #[serde(default)]
    description: Option<DescriptionField>,
    #[serde(default)]
    authors: Vec<AuthorEntry>,
    covers: Option<Vec<i32>>,
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

    let description = match data.description {
        Some(DescriptionField::String(s)) => Some(s),
        Some(DescriptionField::Object { value }) => Some(value),
        None => None,
    };

    let mut authors = Vec::new();
    for entry in data.authors {
        let author_key = entry.author.key;
        let author_id = author_key.trim_start_matches("/authors/");

        let author_url = format!("https://openlibrary.org/authors/{}.json", author_id);
        if let Ok(resp) = reqwest::get(&author_url).await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(name) = json["name"].as_str() {
                    authors.push(Author {
                        name: name.to_string(),
                        key: author_id.to_string(),
                    });
                }
            }
        }
    }

    let covers = data
        .covers
        .map(|ids| ids.into_iter().filter(|id| *id > 0).map(|id| id as u128).collect());

    let book = Book {
        title: data.title,
        description,
        authors,
        covers,
    };

    Json(book)
}
