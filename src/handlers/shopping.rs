use axum::{extract::Path, Json, http::StatusCode};
use mongodb::Database;
use crate::{models::shopping::{CreateShoppingListRequest, UpdateShoppingListRequest, AddItemRequest, UpdateItemRequest, Item}, repositories::shopping_repo::ShoppingRepo};
use validator::Validate;
use uuid::Uuid;

pub async fn create_shopping_list(
    Json(payload): Json<CreateShoppingListRequest>,
    db: Database,
    user_id: String,
) -> Result<Json<String>, StatusCode> {
    if let Err(e) = payload.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let repo = ShoppingRepo::new(&db);
    let list_id = repo.create_list(&payload.name, &user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(list_id.to_hex()))
}

pub async fn update_shopping_list_name(
    Path(list_id): Path<String>,
    Json(payload): Json<UpdateShoppingListRequest>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    let repo = ShoppingRepo::new(&db);
    repo.update_list_name(&list_id, &payload.name).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json("Shopping list updated".to_string()))
}

pub async fn delete_shopping_list(
    Path(list_id): Path<String>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    let repo = ShoppingRepo::new(&db);
    repo.delete_list(&list_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json("Shopping list deleted".to_string()))
}

pub async fn add_item(
    Path(list_id): Path<String>,
    Json(payload): Json<AddItemRequest>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    let repo = ShoppingRepo::new(&db);
    let item = Item {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        done: false,
    };
    repo.add_item(&list_id, item).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json("Item added".to_string()))
}

pub async fn update_item(
    Path((list_id, item_id)): Path<(String, String)>,
    Json(payload): Json<UpdateItemRequest>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    let repo = ShoppingRepo::new(&db);
    repo.update_item(&list_id, &item_id, payload.name, payload.done).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json("Item updated".to_string()))
}

pub async fn delete_item(
    Path((list_id, item_id)): Path<(String, String)>,
    db: Database,
) -> Result<Json<String>, StatusCode> {
    let repo = ShoppingRepo::new(&db);
    repo.delete_item(&list_id, &item_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json("Item deleted".to_string()))
}
