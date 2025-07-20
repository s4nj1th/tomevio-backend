use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub bio: Option<String>,
}

#[derive(Deserialize)]
struct AuthorResponse {
    name: String,
    #[serde(default)]
    bio: Option<BioField>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum BioField {
    String(String),
    Object { value: String },
}

pub async fn get_author(Path(author_id): Path<String>) -> Json<Author> {
    let url = format!("https://openlibrary.org/authors/{}.json", author_id);

    let response = reqwest::get(&url)
        .await
        .expect("Failed to make request to OpenLibrary");

    let data: AuthorResponse = response
        .json()
        .await
        .expect("Failed to parse OpenLibrary response");

    // Normalize the bio to a string
    let bio = match data.bio {
        Some(BioField::String(s)) => Some(s),
        Some(BioField::Object { value }) => Some(value),
        None => None,
    };

    let author = Author {
        name: data.name,
        bio,
    };

    Json(author)
}
