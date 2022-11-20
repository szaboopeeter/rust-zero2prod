//! /src/routes/subscriptions.rs

use actix_web::{web, HttpResponse};
use sqlx::PgConnection;

pub async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
