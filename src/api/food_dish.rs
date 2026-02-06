#[derive(serde::Deserialize)]
struct FoodDishImageResponse {
    image: String,
}

pub async fn get_food_dish_image() -> String {
    let response = reqwest::get("https://foodish-api.com/api")
        .await
        .unwrap()
        .json::<FoodDishImageResponse>()
        .await
        .unwrap();

    response.image
}
