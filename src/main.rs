use axum::{Router, response::IntoResponse, routing::{delete, get, post, put}};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/Students", get(read_students))
        .route("/api/Students/{id}", get(read_student_by_id))
        .route("/api/Students", post(create_student))
        .route("/api/Students", put(update_student))
        .route("/api/Students", delete(delete_student));

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

async fn read_students() -> impl IntoResponse {
    todo!("Implement");
    "The students"
}

async fn read_student_by_id() -> impl IntoResponse {
    todo!("Implement");
    "The student by id"
}

async fn create_student() -> impl IntoResponse {
    todo!("Implement");
    "Student created"
}

async fn update_student() -> impl IntoResponse {
    todo!("Implement");
    "Student updated"
}

async fn delete_student() -> impl IntoResponse {
    todo!("Implement");
    "The students"
}