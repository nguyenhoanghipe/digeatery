use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    pub id: i64,
    pub name: String,
    pub image_url: String,
}


#[get("/api/dish")]
pub async fn get_dish_list() -> Result<Vec<Dish>, ServerFnError> {
    Ok(vec![
        Dish {
            id: 1,
            name: "beef-pho".to_string(),
            image_url: asset!("/assets/image/beef-pho.jpg").to_string(),
        },
        Dish {
            id: 2,
            name: "creme-caramel".to_string(),
            image_url: asset!("/assets/image/creme-caramel.png").to_string(),
        },
    ])
}