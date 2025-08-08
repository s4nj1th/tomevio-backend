use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub passwd: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}