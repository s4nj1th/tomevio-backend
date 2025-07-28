use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
}

pub async fn get_author(Path(author_id): Path<String>) -> Json<Author> {
    let url = format!("https://openlibrary.org/authors/{}.json", author_id);
    let response = reqwest::get(&url).await.expect("Failed to fetch author");
    let author_json: Value = response.json().await.expect("Failed to parse author JSON");

    let name = author_json["name"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string();

    let bio = match &author_json["bio"] {
        Value::String(s) => Some(s.clone()),
        Value::Object(obj) => obj
            .get("value")
            .and_then(|v| v.as_str().map(|s| s.to_string())),
        _ => None,
    };

    let alternate_names = author_json["alternate_names"].as_array().map(|arr| {
        arr.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect::<Vec<String>>()
    });

    // lifespan handling
    let birth = author_json["birth_date"].as_str();
    let death = author_json["death_date"].as_str();
    let date = author_json["date"].as_str();

    fn extract_year(s: &str) -> Option<String> {
        let re = regex::Regex::new(r"\b(\d{4})\b").ok()?;
        re.captures(s)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
    }

    let lifespan = if let Some(date) = date {
        Some(date.to_string())
    } else {
        let birth_year = birth.and_then(extract_year);
        let death_year = death.and_then(extract_year);
        match (birth_year, death_year) {
            (Some(b), Some(d)) => Some(format!("{b}-{d}")),
            (Some(b), None) => Some(format!("{b}-?")),
            (None, Some(d)) => Some(format!("?-{d}")),
            _ => None,
        }
    };

    // fetch works
    let works_url = format!("https://openlibrary.org/authors/{}/works.json", author_id);
    let works_resp = reqwest::get(&works_url)
        .await
        .expect("Failed to fetch works");
    let works_json: Value = works_resp.json().await.expect("Failed to parse works JSON");

    let works: Vec<Book> = works_json["entries"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|entry| {
            let key = entry["key"]
                .as_str()?
                .trim_start_matches("/works/")
                .to_string();
            let title = entry["title"].as_str()?.to_string();
            Some(Book {
                work_id: key,
                title,
            })
        })
        .collect();

    Json(Author {
        name,
        bio,
        alternate_names,
        works,
        lifespan,
    })
}
