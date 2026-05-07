#[derive(serde::Deserialize)]
struct FoodDishImageResponse {
    image: String,
}

pub async fn get_food_dish_image() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://foodish-api.com/api")
        .await?
        .json::<FoodDishImageResponse>()
        .await?;

    Ok(response.image)
}
