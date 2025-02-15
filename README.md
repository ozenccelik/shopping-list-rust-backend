# shopping-list-rust-backend
 This is a personal project


## Project File Structure

```bash
shopping_list_api/
│── src/
│   ├── main.rs
│   ├── config.rs          # Loads environment variables
│   ├── routes.rs          # Defines API routes
│   ├── handlers/
│   │   ├── auth.rs        # Login & register handlers
│   │   ├── shopping.rs    # CRUD for shopping list
│   ├── models/
│   │   ├── user.rs        # User model
│   │   ├── shopping.rs    # Shopping list model
│   ├── services/
│   │   ├── auth_service.rs  # JWT & auth logic
│   │   ├── shopping_service.rs # Business logic for shopping lists
│   ├── repositories/
│   │   ├── user_repo.rs   # Database operations for users
│   │   ├── shopping_repo.rs # Database operations for shopping lists
│   ├── middleware/
│   │   ├── auth_middleware.rs # Middleware for protecting routes
│   ├── utils/
│   │   ├── hash.rs        # Password hashing
│   ├── validation/
│   │   ├── auth_validator.rs # Input validation for auth
│   │   ├── shopping_validator.rs # Input validation for shopping lists
│── Cargo.toml
│── .env                   # MongoDB URI, JWT Secret

```