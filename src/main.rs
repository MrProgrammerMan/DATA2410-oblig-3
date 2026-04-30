use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, prelude::FromRow, query_as};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new().connect(&database_url).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    let app = Router::new()
        .route("/api/Students", get(read_students))
        .route("/api/Students/{id}", get(read_student_by_id))
        .route("/api/Students", post(create_student))
        .route("/api/Students/{id}", put(update_student))
        .route("/api/Students/{id}", delete(delete_student_by_id))
        .route(
            "/api/Students/calculate-grades",
            post(calculate_grades_handler),
        )
        .route("/api/Students/report", get(report_handler))
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
    marks: i32, // Should be custom Marks type
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
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("No student with id {user_id}"),
        ))?;

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
    let result =
        sqlx::query("UPDATE students SET name = $1, course = $2, marks = $3 WHERE id = $4")
            .bind(&new_student.name)
            .bind(&new_student.course)
            .bind(new_student.marks)
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            format!("No student with id {user_id}"),
        ));
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
        return Err((
            StatusCode::NOT_FOUND,
            format!("No student with id {user_id}"),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

/// # Task 1(endpoint 6): Calculate grades
/// Calculates the grades of all students in the database and sets that grade.
/// 
/// Fetches all students, then for each student:
/// 1. Calculates the grade
/// 2. Sets that grade with an update query
async fn calculate_grades_handler(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Student>>, (StatusCode, String)> {
    let students = query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for student in students {
        let grade = calculate_grade(student.marks);
        sqlx::query("UPDATE students SET grade = $1 WHERE id = $2")
            .bind(grade)
            .bind(student.id)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let students = query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(students))
}

/// # Calculate grades(helper)
/// Maps marks to grades "A", "B", "C" or "D"
fn calculate_grade(marks: i32) -> String {
    if marks < 60 {
        "D".to_string()
    } else if marks < 75 {
        "C".to_string()
    } else if marks < 90 {
        "B".to_string()
    } else {
        "A".to_string()
    }
}

#[derive(Serialize)]
struct GradeDistribution {
    #[serde(rename = "A")]
    a: i64,
    #[serde(rename = "B")]
    b: i64,
    #[serde(rename = "C")]
    c: i64,
    #[serde(rename = "D")]
    d: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Report {
    course_name: String,
    total_students: i64,
    average_marks: f64,
    grade_distribution: GradeDistribution,
}

// Intern row-type fra databasen (FromRow, ikke direkte serialisert)
#[derive(FromRow)]
struct ReportRow {
    course: String,
    total_students: i64,
    average_marks: f64,
    grade_a_count: i64,
    grade_b_count: i64,
    grade_c_count: i64,
    grade_d_count: i64,
}

async fn report_handler(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Report>>, (StatusCode, String)> {
    let rows = query_as::<_, ReportRow>(
        r#"
        SELECT
            course,
            COUNT(*)                                        AS total_students,
            ROUND(AVG(marks), 2)::FLOAT8                   AS average_marks,
            COUNT(*) FILTER (WHERE grade = 'A')            AS grade_a_count,
            COUNT(*) FILTER (WHERE grade = 'B')            AS grade_b_count,
            COUNT(*) FILTER (WHERE grade = 'C')            AS grade_c_count,
            COUNT(*) FILTER (WHERE grade = 'D')            AS grade_d_count
        FROM students
        GROUP BY course
        ORDER BY average_marks DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let report = rows
        .into_iter()
        .map(|r| Report {
            course_name: r.course,
            total_students: r.total_students,
            average_marks: r.average_marks,
            grade_distribution: GradeDistribution {
                a: r.grade_a_count,
                b: r.grade_b_count,
                c: r.grade_c_count,
                d: r.grade_d_count,
            },
        })
        .collect();

    Ok(Json(report))
}
