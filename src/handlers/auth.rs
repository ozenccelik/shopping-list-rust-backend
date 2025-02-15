use axum::{Json, http::StatusCode};
use mongodb::Database;
use crate::models::user::{RegisterRequest, LoginRequest, User};
use crate::services::auth_service::{hash_password, login};
use validator::Validate;

pub async fn register(
    Json(payload): Json<RegisterRequest>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    if let Err(e) = payload.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = User {
        id: None,
        email: payload.email,
        password: hash_password(&payload.password),
    };

    let collection = db.collection("users");
    collection.insert_one(user, None).await.unwrap();

    Ok(Json("User registered successfully".to_string()))
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    match login(&db, payload).await {
        Some(token) => Ok(Json(token)),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
