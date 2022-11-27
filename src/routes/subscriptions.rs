//! /src/routes/subscriptions.rs

use chrono::Utc;
use uuid::Uuid;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Saving new subscriber details in the database", 
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    // TODO: Should not use `enter()` inside async functions
    // As long as the guard variable is not dropped, all downstream spans
    // will be the children of the entered span
    // RAII pattern: https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html
    let _request_span_guard = request_span.enter();

    // `Result` has two variants: `Ok` and `Err`.
    // The first for successes, the second for failures.
    // We use a `match` statement to choose what to do based 
    // on the outcome.
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await 
    {
        Ok(_) =>{ 
            tracing::info!("request_id {} - Adding '{}' '{}' as a new subscriber", request_id, form.email, form.name);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
