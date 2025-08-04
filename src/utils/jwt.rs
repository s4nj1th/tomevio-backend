use axum::http::StatusCode;
use chrono::{Duration, Utc};
use crates::utils;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

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
        email: email,
    };
    let secret = (*utils::constants::TOKEN).clone();

    return encode(
        &header::default(),
        &claim,
        &EncodingKey::from_string(secre.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}

pub fn decode(jwt: String) -> Result<TokenData<Claim>, StatusCode> {
    let secret = (*utils::constants::TOKEN).clone();
    let res = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    return res;
}
