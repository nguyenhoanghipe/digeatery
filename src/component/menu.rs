use crate::api::get_food_dish_image;
use crate::api::post_menu;
use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    // let mut img_src_resource =
    //     use_resource(move || async move { get_food_dish_image().await.unwrap_or_default() });

    use_effect(move || {
        spawn(async move {
            match get_food_dish_image().await {
                Ok(src) => img_src.set(src),
                Err(e) => eprintln!("Error fetching food dish image: {}", e),
            }
        });
    });

    rsx! {
        div {
            img { src: "{img_src}" }
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button {
                id: "save",
                onclick: move |_| async move {
                    _ = post_menu(img_src.cloned()).await;
                },

                "save!"
            }
        }
    }
}
