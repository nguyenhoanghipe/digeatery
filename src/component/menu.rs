use crate::api::{get_dish_list};
use dioxus::prelude::*;


#[component]
pub fn Menu() -> Element {
    let dishes = use_resource(get_dish_list);

    let abc = get_dish_list();

    reqwest::get("https://www.google.com/");

use_effect(|| {
    println!("EFFECT RUNNING");

    spawn(async {
        println!("BEFORE REQUEST");

        let response = reqwest::get("https://www.google.com/").await;

        println!("AFTER REQUEST: {response:?}");
    });
});

    println!("MENU COMPONENT RENDERED");
    rsx! {
        // img { src: asset!("/assets/image/beef-pho.jpg").to_string() }
        // img { src: asset!("/assets/image/creme-caramel.png").to_string() }
        match &*dishes.read() {
            Some(Ok(items)) => rsx! {
                for item in items {
                    div {
                        h3 { "{item.name}" }

                        img { src: "{item.image_url}", alt: "{item.name}" }
                    }
                }
            },
            Some(Err(error)) => rsx! {
                p { "Error: {error}" }
            },

            None => rsx! {
                p { "Loading..." }
            },
        }
    }
}
