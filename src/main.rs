use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct Student {
    id: i32,
    name: String,
    course: String,
    marks: i32, // Should be custom marks type
    grade: Option<String> // Should be Option<char>
}