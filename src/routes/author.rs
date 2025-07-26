use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub bio: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    pub works: Vec<Book>,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub work_id: String,
    pub title: String,
    // pub covers: Option<Vec<u128>>,
}

#[derive(Deserialize)]
struct AuthorResponse {
    name: String,
    #[serde(default)]
    bio: Option<BioField>,
    alternate_names: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum BioField {
    String(String),
    Object { value: String },
}

#[derive(Deserialize)]
struct WorksResponse {
    entries: Vec<WorkEntry>,
}

#[derive(Deserialize)]
struct WorkEntry {
    key: String,
    title: String,
    // #[serde(default)]
    // covers: Option<Vec<u128>>,
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

    let bio = match data.bio {
        Some(BioField::String(s)) => Some(s),
        Some(BioField::Object { value }) => Some(value),
        None => None,
    };

    let alternate_names = data.alternate_names.and_then(|val| {
        val.as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
    });
    
    let works_url = format!("https://openlibrary.org/authors/{}/works.json", author_id);
    let works_response = reqwest::get(&works_url)
        .await
        .expect("Failed to fetch author's works");
    let works_data: WorksResponse = works_response
        .json()
        .await
        .expect("Failed to parse works JSON");

    let works: Vec<Book> = works_data
        .entries
        .into_iter()
        .map(|entry| Book {
            work_id: entry.key.trim_start_matches("/works/").to_string(),
            title: entry.title,
            // covers: entry.covers,
        })
        .collect();

    let author = Author {
        name: data.name,
        bio,
        alternate_names,
        works,
    };

    Json(author)
}
