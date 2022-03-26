use actix_web::{web, HttpResponse};
use sqlx::{query, PgConnection, PgPool};

use serde::Deserialize;
use serde::Serialize;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GlobalSettings {
    pub id: Uuid,
    pub fuel_price: BigDecimal,
    pub diesel_price: BigDecimal,
}

pub async fn get_global_settings(pool: web::Data<PgPool>) -> HttpResponse {
    let saved = sqlx::query!("SELECT id,fuel_price,diesel_price FROM global_settings",)
        .fetch_one(pool.get_ref())
        .await
        .expect("Failed to fetch saved subscription.");

    let global_settings = GlobalSettings {
        id: saved.id,
        fuel_price: saved.fuel_price,
        diesel_price: saved.diesel_price,
    };
    HttpResponse::Ok().json(global_settings)
}
