use crate::api::get_available_order_date_list;
use crate::common::ui::radio_group::{RadioGroup, RadioItem};
use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    let abc = use_resource(get_available_order_date_list);

    rsx! {
        RadioGroup {
            match &*abc.read() {
                Some(Ok(items)) => rsx! {
                    for item in items {
                        RadioItem { value: item.to_string(), index: item.to_julian_day() as usize, {item.to_string()} }
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

        // img { src: asset!("/asset/image/beef-pho.jpg").to_string() }
        // img { src: asset!("/asset/image/creme-caramel.png").to_string() }
    }
}
