use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingList {
    pub id: Option<String>,
    pub name: String,
    pub user_id: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateShoppingListRequest {
    #[validate(length(min = 3, message = "Shopping list name must be at least 3 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateShoppingListRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AddItemRequest {
    #[validate(length(min = 2, message = "Item name must be at least 2 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub done: Option<bool>,
}
