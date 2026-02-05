use dioxus::prelude::*;
use crate::components::SignIn;

pub fn Home() -> Element {
    rsx! {
        SignIn {}
    }
}