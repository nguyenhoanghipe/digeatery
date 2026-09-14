use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use time::macros::date;
use time::Date;
use crate::server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    pub id: i64,
    pub name: String,
    pub image_url: String,
}

#[get("/server/order-date")]
pub async fn get_available_order_date_list() -> Result<Vec<Date>, ServerFnError> {
    // let db = server::get_db().await.map_err(|e| e.into())?;


    Ok(vec![
        date!(2026 - 09 - 04),
        date!(2026 - 09 - 06),
        date!(2026 - 09 - 10),
    ])
}
