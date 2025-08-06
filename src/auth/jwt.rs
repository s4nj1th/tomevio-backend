use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Claim {
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
        email,
    };

    let secret = std::env::var("TOKEN")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claim>, StatusCode> {
    let secret = std::env::var("TOKEN")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    decode::<Claim>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
