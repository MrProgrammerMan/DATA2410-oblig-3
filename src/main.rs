use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::Serialize;

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

#[derive(Serialize)]
struct Student {
    id: i32,
    name: String,
    course: String,
    marks: i32,            // Should be custom marks type
    grade: Option<String>, // Should be Option<char>
}

async fn read_students() -> Json<Vec<Student>> {
    Json(vec![
        Student {
            id: 1,
            name: "John".to_string(),
            course: "DATA2410".to_string(),
            marks: 23,
            grade: None,
        },
        Student {
            id: 2,
            name: "Lisa".to_string(),
            course: "DATA2300".to_string(),
            marks: 21,
            grade: Some("D".to_string()),
        },
        Student {
            id: 3,
            name: "Igor".to_string(),
            course: "Mathematics 1".to_string(),
            marks: 90,
            grade: None,
        },
        Student {
            id: 4,
            name: "Chris".to_string(),
            course: "DAPE2101".to_string(),
            marks: 10,
            grade: Some("A".to_string()),
        },
    ])
}

async fn read_student_by_id() -> Json<Student> {
    Json(Student {
        id: 2,
        name: "Lisa".to_string(),
        course: "DATA2300".to_string(),
        marks: 21,
        grade: Some("D".to_string()),
    })
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
