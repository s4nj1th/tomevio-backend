use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use jsonwebtoken::TokenData;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{
    auth::jwt::{decode_jwt, Claim},
    entity::user,
};

pub async fn guard(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

        if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let token = auth_header.trim_start_matches("Bearer ").to_string();

    let token_data: TokenData<Claim> = decode_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let email = token_data.claims.email.to_lowercase();

    let db = req
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let identity = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
