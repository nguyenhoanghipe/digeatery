use crate::component::Menu;
use crate::component::SignIn;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        SignIn {}

     Menu {}
    }
}
