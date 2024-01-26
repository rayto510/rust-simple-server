use axum::{extract::Query, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

async fn handler(query_params: Query<Person>) -> Json<Value> {
    let Person { name, age } = query_params.0;

    Json(json!({ "data": format!("Hello, {}! You are {} years old.", name, age) }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/person", get(handler))
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
