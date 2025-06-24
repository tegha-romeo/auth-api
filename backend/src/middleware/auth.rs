use crate::config::config::get_jwt_secret;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::db::queries::get_user_by_email;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_token(email: &str, role: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(10))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

    let secret = get_jwt_secret();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = get_jwt_secret();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub async fn auth_middleware(
    State(pool): State<Pool<Postgres>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let token = auth.token();
    let claims = decode_token(token).map_err(|e| {
        let message = format!("Invalid token: {}", e);
        (StatusCode::UNAUTHORIZED, message)
    })?;

    let user = get_user_by_email(&pool, &claims.sub)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

pub struct AuthUser(pub User);

impl AuthUser {
    pub fn get_user(req: &Request<axum::body::Body>) -> Option<&User> {
        req.extensions().get::<User>()
    }
}
