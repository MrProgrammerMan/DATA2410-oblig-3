use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, prelude::FromRow, query_as};

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:postgres@localhost:5432/db";
    let db = PgPoolOptions::new().connect(database_url).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    let app = Router::new()
        .route("/api/Students", get(read_students))
        .route("/api/Students/{id}", get(read_student_by_id))
        .route("/api/Students", post(create_student))
        .route("/api/Students", put(update_student))
        .route("/api/Students", delete(delete_student))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, FromRow)]
struct Student {
    id: i32,
    name: String,
    course: String,
    marks: i32,            // Should be custom marks type
    grade: Option<String>, // Should be Option<char>
}

#[derive(Deserialize)]
struct NewStudent {
    name: String,
    course: String,
    marks: i32,            // Should be custom marks type
    grade: Option<String>, // Should be Option<char>
}

async fn read_students(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Student>>, (StatusCode, String)> {
    let students = query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(students))
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

async fn create_student(
    State(pool): State<Pool<Postgres>>,
    Json(new_student): Json<NewStudent>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("INSERT INTO students(name, course, marks, grade) VALUES ($1, $2, $3, $4)")
        .bind(&new_student.name)
        .bind(&new_student.course)
        .bind(new_student.marks)
        .bind(&new_student.grade)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn update_student() -> impl IntoResponse {
    todo!("Implement");
    "Student updated"
}

async fn delete_student() -> impl IntoResponse {
    todo!("Implement");
    "The students"
}
