use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

async fn occupy(
    State(state): State<MyState>,
    Json(data): Json<OccupyJsonBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if data.email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Email cannot be empty".to_string()));
    }

    match sqlx::query("UPDATE timeslots SET email = $1 WHERE id = $2 AND email IS NULL RETURNING id")
        .bind(&data.email)
        .bind(data.id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(_)) => Ok(StatusCode::OK),
        Ok(None) => Err((StatusCode::CONFLICT, "Timeslot already occupied".to_string())),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some("23505") {
                    return Err((StatusCode::CONFLICT, "User already has a timeslot".to_string()));
                }
            }
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        },
    }
}

async fn get_all(
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Timeslot>("SELECT * FROM timeslots")
        .fetch_all(&state.pool)
        .await
    {
        Ok(timeslots) => Ok((StatusCode::OK, Json(timeslots))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_by_email(
    Query(params): Query<GetByEmailParams>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Timeslot>("SELECT * FROM timeslots WHERE email = $1")
        .bind(params.email)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(timeslot) => Ok((StatusCode::OK, Json(timeslot))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/timeslots/occupy", post(occupy))
        .route("/timeslots/getByEmail", get(get_by_email))
        .route("/timeslots", get(get_all))
        .fallback_service(ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state);

    Ok(router.into())
}

#[derive(Deserialize)]
struct OccupyJsonBody {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize, FromRow)]
struct Timeslot {
    pub id: i32,
    pub email: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub end_time: time::OffsetDateTime,
}

#[derive(Deserialize)]
struct GetByEmailParams {
    pub email: String,
}
