use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

async fn occupy(
    State(state): State<MyState>,
    Json(data): Json<OccupyJsonBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query("UPDATE timeslots SET email = $1 WHERE id = $2")
        .bind(&data.email)
        .bind(data.id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
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
    pub start_time: time::OffsetDateTime,
    pub end_time: time::OffsetDateTime,
}

#[derive(Deserialize)]
struct GetByEmailParams {
    pub email: String,
}
