use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub bio: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    pub works: Vec<Book>,
    pub lifespan: Option<String>,
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
    #[serde(default)]
    birth_date: Option<String>,
    #[serde(default)]
    death_date: Option<String>,
    #[serde(default)]
    date: Option<String>,
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

    fn extract_year(s: &str) -> Option<String> {
        let re = regex::Regex::new(r"\b(\d{4})\b").ok()?;
        re.captures(s)
            .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    }

    let lifespan = if data.date.is_some() {
        data.date.clone()
    } else if data.birth_date.is_some() || data.death_date.is_some() {
        let birth_year = data.birth_date.as_ref().and_then(|d| extract_year(d));
        let death_year = data.death_date.as_ref().and_then(|d| extract_year(d));

        match (birth_year, death_year) {
            (Some(b), Some(d)) => Some(format!("{b}-{d}")),
            (Some(b), None) => Some(format!("{b}-?")),
            (None, Some(d)) => Some(format!("?-{d}")),
            _ => None,
        }
    } else {
        None
    };

    let author = Author {
        name: data.name,
        bio,
        alternate_names,
        works,
        lifespan,
    };

    Json(author)
}
