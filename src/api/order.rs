use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
use serde::{Deserialize, Serialize};
use time::macros::date;
use time::Date;
#[cfg(feature = "server")]
use crate::server::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    pub id: i64,
    pub name: String,
    pub image_url: String,
}

#[get("/api/order-date")]
pub async fn get_available_order_date_list() -> Result<Vec<Date>, ServerFnError> {
    // let db = server::get_db().await.map_err(|e| e.into())?;


    Ok(vec![
        date!(2026 - 09 - 04),
        date!(2026 - 09 - 06),
        date!(2026 - 09 - 10),
    ])
}

#[post("/api/order-date", ext: Extension<State>)]
pub async fn add_available_order_date(date: Date) -> Result<i32, ServerFnError> {
    let db = &ext.db;

    let id = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO fulfillment_date (date)
        VALUES ($1)
        RETURNING id
        "#,
    )
    .bind(date)
    .fetch_one(db)
    .await
    .map_err(ServerFnError::new)?;

    Ok(id)
}
