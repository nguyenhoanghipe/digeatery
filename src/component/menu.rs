use crate::api::get_food_dish_image;
use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    let save = async move |_| {
        let src = get_food_dish_image().await.unwrap();
        img_src.set(src);
    };

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            // ..
            button { onclick: save, id: "save", "save!" }
        }
    }
}
