use dioxus::prelude::*;

#[get("/api/menu")]
pub async fn get_menu() -> Result<String, ServerFnError> {
    Ok("test".to_string())
}

#[post("/api/menu")]
pub async fn post_menu(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `menu.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("menu.txt")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"))
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
