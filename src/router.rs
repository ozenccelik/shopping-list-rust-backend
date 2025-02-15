use axum::{Router, routing::post};
use crate::handlers::{auth::register, auth::login};

pub fn create_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))

        .route("/shopping-list", post(create_shopping_list))
        .route("/shopping-list/:id", put(update_shopping_list_name).delete(delete_shopping_list))
        .route("/shopping-list/:id/item", post(add_item))
        .route("/shopping-list/:id/item/:item_id", put(update_item).delete(delete_item))
}