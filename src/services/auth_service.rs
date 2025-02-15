use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{bson::doc, Collection, Database};
use argon2::{self, Config};
use crate::models::user::{User, LoginRequest};
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET: &str = "your_secret_key";

pub fn hash_password(password: &str) -> String {
    let salt = b"random_salt";
    argon2::hash_encoded(password.as_bytes(), salt, &Config::default()).unwrap()
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    argon2::verify_encoded(hashed, password.as_bytes()).unwrap_or(false)
}

pub async fn login(db: &Database, credentials: LoginRequest) -> Option<String> {
    let user_collection: Collection<User> = db.collection("users");
    let user = user_collection.find_one(doc! {"email": &credentials.email}, None).await.ok()?;

    if let Some(user) = user {
        if verify_password(&credentials.password, &user.password) {
            return Some(generate_jwt(&user.email));
        }
    }
    None
}

fn generate_jwt(email: &str) -> String {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600;
    let claims = serde_json::json!({ "email": email, "exp": expiration });
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref())).unwrap()
}
