use axum::{routing::get, Json, Router};
mod model;
use model::Todo;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/todos", get(list_todos));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn list_todos() -> Json<Vec<Todo>> {
    let todos = vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Build API".to_string(),
            completed: true,
        },
    ];
    Json(todos)
}
