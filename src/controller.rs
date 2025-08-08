use crate::model::{Claims, LoginInfo, LoginResponse};
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub async fn login_handler(
    Json(login_info): Json<LoginInfo>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let username = &login_info.username;
    let passwd = &login_info.passwd;

    let is_valid = is_valid_user(username, passwd);

    if is_valid {
        let claim = Claims {
            sub: username.clone(),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = match encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret("secret_key".as_ref()),
        ) {
            Ok(token) => token,
            Err(e) => {
                eprint!("Error encoding token {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        Ok(Json(LoginResponse { token }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub fn is_valid_user(username: &str, passwd: &str) -> bool {
    // TODO: Implement actual user validation logic
    username == "admin" && passwd == "password"
}

pub async fn get_info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret("secret_key".as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => {
                        let info = "You are valid".to_string();
                        return Ok(Json(info));
                    }
                    Err(_) => return Err(StatusCode::UNAUTHORIZED),
                }
            } else {
                eprintln!("Invalid Authorization header format");
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
