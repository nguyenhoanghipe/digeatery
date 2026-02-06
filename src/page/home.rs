use dioxus::prelude::*;
use crate::component::SignIn;

pub fn Home() -> Element {
    rsx! {
        SignIn {}
    }
}