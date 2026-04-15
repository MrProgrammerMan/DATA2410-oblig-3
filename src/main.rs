use axum::{
    Json, Router,
    extract::{Path, State},
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
        .route("/api/Students/{id}", put(update_student))
        .route("/api/Students/{id}", delete(delete_student_by_id))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, FromRow)]
struct Student {
    id: i32,
    name: String,
    course: String,
    marks: i32,            // Should be custom Marks type
    grade: Option<String>, // Should be Option<Grade>
}

#[derive(Deserialize)]
struct NewStudent {
    name: String,
    course: String,
    marks: i32,            // Should be custom Marks type
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

async fn read_student_by_id(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Json<Student>, (StatusCode, String)> {
    let student = query_as::<_, Student>("SELECT * FROM students WHERE id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(student))
}

async fn create_student(
    State(pool): State<Pool<Postgres>>,
    Json(new_student): Json<NewStudent>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("INSERT INTO students(name, course, marks) VALUES ($1, $2, $3)")
        .bind(&new_student.name)
        .bind(&new_student.course)
        .bind(new_student.marks)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn update_student(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i32>,
    Json(new_student): Json<NewStudent>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query(
        "UPDATE students SET name = $1, course = $2, marks = $3 WHERE id = $4"
    )
        .bind(&new_student.name)
        .bind(&new_student.course)
        .bind(new_student.marks)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, format!("No student with id {user_id}")));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_student_by_id(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM students WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, format!("No student with id {user_id}")));
    }

    Ok(StatusCode::NO_CONTENT)
}