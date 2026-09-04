use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use time::macros::date;
use time::Date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    pub id: i64,
    pub name: String,
    pub image_url: String,
}

#[get("/api/order-date")]
pub async fn get_available_order_date_list() -> Result<Vec<Date>, ServerFnError> {
    Ok(vec![
        date!(2026 - 09 - 04),
        date!(2026 - 09 - 06),
        date!(2026 - 09 - 10),
    ])
}
