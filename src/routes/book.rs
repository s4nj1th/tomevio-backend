use axum::{extract::Path, Json};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Book {
    pub title: String,
    pub description: Option<String>,
    pub authors: Vec<Author>,
    pub edition_key: Option<String>,
    pub publish_date: Option<String>,
    pub number_of_pages: Option<u32>,
    pub cover: Option<u32>,
}

#[derive(Serialize)]
pub struct Author {
    pub name: String,
    pub key: String,
}

pub async fn get_book(Path(id): Path<String>) -> Json<Book> {
    let (is_work, raw_id) = if id.starts_with("OL") && id.ends_with("M") {
        (false, id.as_str())
    } else {
        (true, id.as_str())
    };

    if is_work {
        get_book_from_work(raw_id).await
    } else {
        get_book_from_edition(raw_id).await
    }
}

pub async fn get_book_from_edition(id: &str) -> Json<Book> {
    let edition_url = format!("https://openlibrary.org/books/{}.json", id);
    let edition_resp = reqwest::get(&edition_url)
        .await
        .expect("Failed to fetch edition");
    let edition_json: Value = edition_resp.json().await.expect("Failed to parse edition");

    let title = edition_json["title"]
        .as_str()
        .unwrap_or("Untitled")
        .to_string();

    let edition_key = Some(id.to_string());
    let publish_date = edition_json["publish_date"].as_str().map(|s| s.to_string());
    let number_of_pages = edition_json["number_of_pages"].as_u64().map(|n| n as u32);

    let work_id = edition_json["works"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|w| w["key"].as_str())
        .map(|s| s.trim_start_matches("/works/").to_string());

    let (description, authors) = if let Some(work_id) = work_id {
        fetch_work_details(&work_id).await
    } else {
        (None, vec![])
    };

    let cover = edition_json["covers"].as_array().and_then(|arr| {
        arr.iter()
            .filter_map(|v| v.as_i64())
            .find(|&n| n >= 0)
            .map(|n| n as u32)
    });

    Json(Book {
        title,
        description,
        authors,
        edition_key,
        publish_date,
        number_of_pages,
        cover,
    })
}

pub async fn get_book_from_work(id: &str) -> Json<Book> {
    let work_url = format!("https://openlibrary.org/works/{}.json", id);
    let work_resp = reqwest::get(&work_url).await.expect("Failed to fetch work");
    let work_json: Value = work_resp.json().await.expect("Failed to parse work");

    let title = work_json["title"]
        .as_str()
        .unwrap_or("Untitled")
        .to_string();

    let description = work_json["description"]
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            work_json["description"]["value"]
                .as_str()
                .map(|s| s.to_string())
        });

    let authors = if let Some(author_entries) = work_json["authors"].as_array() {
        let mut authors = vec![];
        for entry in author_entries {
            if let Some(author_key) = entry["author"]["key"].as_str() {
                let author_id = author_key.trim_start_matches("/authors/");
                let author_url = format!("https://openlibrary.org/authors/{}.json", author_id);
                if let Ok(resp) = reqwest::get(&author_url).await {
                    if let Ok(json) = resp.json::<Value>().await {
                        if let Some(name) = json["name"].as_str() {
                            authors.push(Author {
                                name: name.to_string(),
                                key: author_id.to_string(),
                            });
                        }
                    }
                }
            }
        }
        authors
    } else {
        vec![]
    };

    let cover = work_json["covers"].as_array().and_then(|arr| {
        arr.iter()
            .filter_map(|v| v.as_i64())
            .find(|&n| n >= 0)
            .map(|n| n as u32)
    });

    Json(Book {
        title,
        description,
        authors,
        edition_key: None,
        publish_date: None,
        number_of_pages: None,
        cover,
    })
}

async fn fetch_work_details(work_id: &str) -> (Option<String>, Vec<Author>) {
    let work_url = format!("https://openlibrary.org/works/{}.json", work_id);
    if let Ok(work_resp) = reqwest::get(&work_url).await {
        if let Ok(work_json) = work_resp.json::<Value>().await {
            let description = work_json["description"]
                .as_str()
                .map(|s| s.to_string())
                .or_else(|| {
                    work_json["description"]["value"]
                        .as_str()
                        .map(|s| s.to_string())
                });

            let mut authors = vec![];

            if let Some(author_entries) = work_json["authors"].as_array() {
                for entry in author_entries {
                    if let Some(author_key) = entry["author"]["key"].as_str() {
                        let author_id = author_key.trim_start_matches("/authors/");
                        let author_url =
                            format!("https://openlibrary.org/authors/{}.json", author_id);
                        if let Ok(resp) = reqwest::get(&author_url).await {
                            if let Ok(json) = resp.json::<Value>().await {
                                if let Some(name) = json["name"].as_str() {
                                    authors.push(Author {
                                        name: name.to_string(),
                                        key: author_id.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }

            return (description, authors);
        }
    }

    (None, vec![])
}
