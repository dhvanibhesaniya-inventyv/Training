use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post, put},
    extract::Path,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
}

lazy_static::lazy_static! {
    static ref USERS: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(Vec::new()));
}

async fn create_user() -> impl IntoResponse {
    // Reset user data by clearing the vector and pushing new data
    let mut users = USERS.write().unwrap();
    users.push(User { id: 1, name: "Elijah".to_string(), email: "elijah@example.com".to_string() });
    users.push(User { id: 2, name: "John".to_string(), email: "john@example.com".to_string() });
    users.push(User { id: 3, name: "Alice".to_string(), email: "alice@example.com".to_string() });
    users.push(User { id: 4, name: "Bob".to_string(), email: "bob@example.com".to_string() });
    users.push(User { id: 5, name: "Charlie".to_string(), email: "charlie@example.com".to_string() });

    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User data entered successfully"))
        .unwrap()
}

async fn list_users() -> Json<Vec<User>> {
    let users = USERS.read().unwrap(); // Use read lock for reading the users
    Json(users.to_vec())
}

async fn delete_user(Path(id): Path<u64>) -> Json<Vec<User>> {
    let mut users = USERS.write().unwrap();
    if let Some(index) = users.iter().position(|user| user.id == id) {
        users.remove(index);
    } else {
        // Return the current state of the users vector if the user is not found
        return Json(users.to_vec());
    }

    Json(users.to_vec())
}

async fn update_user(Path((id, name, email)): Path<(u64, String, String)>) -> impl IntoResponse {
    let mut users = USERS.write().unwrap();
    if let Some(user) = users.iter_mut().find(|user| user.id == id) {
        user.name = name;
        user.email = email;
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("Updated user with id {}", id)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("User with id {} not found", id)))
            .unwrap()
    }
}

#[tokio::main]
pub async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        .route("/delete-user/:id", delete(delete_user))
        .route("/update-user/:id/:name/:email", put(update_user));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
