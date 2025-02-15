use axum::{
    routing::{get, post},
    Router, Json, extract::{Path, Query}, http::StatusCode,
};
use mongodb::{Client, Database};
use std::{env, sync::Arc};
use tower_http::{
    compression::CompressionLayer,
    cors::{CorsLayer, Any},
    rate_limit::RateLimitLayer,
};
use tower::{ServiceBuilder, layer::util::Identity};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use tokio::sync::RwLock;
use axum::middleware::{self, Next};
use axum::http::Request;

// MongoDB document structure
#[derive(Serialize, Deserialize, Clone)]
struct Item {
    name: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct ShoppingList {
    name: String,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    // Initialize environment variables and database connection
    dotenvy::dotenv().ok();
    let db = init_db().await;

    // Set up Axum app
    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/shopping-list", post(create_shopping_list))
        .route("/shopping-list/:id", put(update_shopping_list_name))
        .route("/shopping-list/:id", delete(delete_shopping_list))
        .route("/shopping-list/:id/item", post(insert_item))
        .route("/shopping-list/:id/item/:item_id", put(update_item))
        .route("/shopping-list/:id/item/:item_id", delete(delete_item))
        // Custom middleware logging
        .layer(ServiceBuilder::new().layer(log_request))
        .layer(ServiceBuilder::new()
            .layer(Identity::new())  // Using Identity from tower for simple request logging
            .layer(CompressionLayer::new())    // Enable response compression (gzip, brotli)
            .layer(CorsLayer::new().allow_origin(Any)) // Enable CORS
            .layer(RateLimitLayer::new(100, std::time::Duration::from_secs(60))) // Rate Limiting: 100 requests/min
        );

    let addr = "127.0.0.1:3000".parse().unwrap();
    info!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// MongoDB connection initialization
async fn init_db() -> Database {
    let client = Client::with_uri_str(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    client.database("shopping_list_db")
}

// Root handler
async fn root() -> &'static str {
    "Hello, World! This is the Shopping List API."
}

// Register user handler
async fn register_user(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    // Here, you should hash the password and store it in your database
    Json(RegisterResponse {
        message: format!("User {} registered successfully", payload.username),
    })
}

// Login user handler (JWT Token generation)
async fn login_user(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    // Validate user credentials, for now, we're not actually validating the password
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let claims = vec!["claim1", "claim2"]; // Example claims
    let header = Header::default();

    // Create JWT token
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_bytes()))
        .expect("Failed to create token");

    Json(LoginResponse { token })
}

// Create shopping list handler
async fn create_shopping_list(Json(payload): Json<ShoppingList>) -> Json<ShoppingList> {
    // Here, you would store the shopping list in MongoDB
    Json(payload)
}

// Update shopping list name handler
async fn update_shopping_list_name(
    Path(id): Path<String>, 
    Json(payload): Json<ShoppingList>
) -> Result<Json<ShoppingList>, StatusCode> {
    // Update shopping list name in the database by id
    // For now, just simulate it
    Ok(Json(payload))
}

// Delete shopping list handler
async fn delete_shopping_list(Path(id): Path<String>) -> StatusCode {
    // Delete shopping list by id
    // Simulate successful deletion
    StatusCode::NO_CONTENT
}

// Insert item into shopping list handler
async fn insert_item(Path(id): Path<String>, Json(payload): Json<Item>) -> Json<Item> {
    // Insert item into the shopping list in the database by id
    Json(payload)
}

// Update item handler
async fn update_item(
    Path((id, item_id)): Path<(String, String)>, 
    Json(payload): Json<Item>
) -> Json<Item> {
    // Update item in the shopping list by id and item_id
    Json(payload)
}

// Delete item handler
async fn delete_item(Path((id, item_id)): Path<(String, String)>) -> StatusCode {
    // Delete item in the shopping list by id and item_id
    StatusCode::NO_CONTENT
}
