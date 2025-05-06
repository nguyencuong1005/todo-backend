use axum::{routing::get, Json, Router};
use postgres::{Client, Error, NoTls};

#[tokio::main]
async fn main() {
    let db = db_connect().unwrap();
    let app = Router::new().route("/", get(root_handler)).with_state(db); // Pass the database client to handlers

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn db_connect() -> Result<Client, Error> {
    let mut client = Client::connect("postgresql://postgres:53r45331@localhost:5432/todo", NoTls)?;

    // client.batch_execute(
    //     "
    //     CREATE TABLE IF NOT EXISTS tasks (
    //         id              SERIAL PRIMARY KEY,
    //         title           VARCHAR NOT NULL,
    //         completed       BOOLEAN NOT NULL DEFAULT FALSE
    //     )
    //     ",
    // )?;

    Ok(client)
}

async fn root_handler() -> Json<String> {
    Json(format!("Hello, World!"))
}
