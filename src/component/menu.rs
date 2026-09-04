use crate::api::{get_available_order_date_list, get_dish_list};
use crate::common::ui::radio_group::{RadioGroup, RadioItem};
use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    let abc = use_resource(get_available_order_date_list);

    rsx! {
        RadioGroup {
           RadioItem  { value : "", index: 1usize, "abc" }
            RadioItem { value: "", index: 3usize, "1223"}
            RadioItem { value: "", index: 3usize, "1234"}
        }

        // img { src: asset!("/assets/image/beef-pho.jpg").to_string() }
        // img { src: asset!("/assets/image/creme-caramel.png").to_string() }
        // match &*dishes.read() {
        //     Some(Ok(items)) => rsx! {
        //         for item in items {
        //             div {
        //                 h3 { "{item.name}" }
        //
        //                 img { src: "{item.image_url}", alt: "{item.name}" }
        //             }
        //         }
        //     },
        //     Some(Err(error)) => rsx! {
        //         p { "Error: {error}" }
        //     },
        //
        //     None => rsx! {
        //         p { "Loading..." }
        //     },
        // }
    }
}
