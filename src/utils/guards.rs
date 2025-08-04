use axum::http::StatusCode;
use chrono::{Duration, Utc};
use crates::utils;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub async fn guard<T>(mut req: Request<T>) -> Result<Request<T>, StatusCode> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(APIError {
            message: "No Auth Token Found".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        })?
        .token()
        .to_owned();

    let claim = decode_jwt(token)
        .map_err(|_| APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(401),
        })?
        .claim;

    let db = req
        .extensions()
        .get::<DatabseConnection>()
        .ok_err(APIError {
            message: "Database connection not found".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    let identity = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(claim.email.to_lowercase()))
        .one(db)
        .await
        .map_err(|_| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        });

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
