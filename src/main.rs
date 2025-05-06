use axum::{
    routing::{get, post},
    Json, Router,
};
use postgres::{Client, Error, NoTls};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db = tokio::task::spawn_blocking(|| db_connect())
        .await
        .unwrap()
        .unwrap();
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/todos", get(create_todo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn db_connect() -> Result<Client, Error> {
    let mut client = Client::connect("postgresql://postgres:53r45331@localhost:5432/todo", NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            completed       BOOLEAN NOT NULL DEFAULT FALSE
        )
        ",
    )?;

    Ok(client)
}

async fn root_handler() -> Json<String> {
    Json("Hello, World!".to_string())
}

async fn create_todo() -> Json<String> {
    let mut client = Client::connect("postgresql://postgres:53r45331@localhost:5432/todo", NoTls);

    let id = 1;
    let title = "Create repo".to_string();
    let completed_status = false;

    client.unwrap().execute(
        "INSERT INTO tasks (id, title, completed) VALUES ($1, $2, $3)",
        &[&&id, &title, &completed_status],
    );

    Json("one".to_string())
}
