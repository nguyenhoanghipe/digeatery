use dioxus::prelude::*;
use crate::component::{Menu, SignIn};

pub fn Home() -> Element {
    rsx! {
        Menu {}
    }
}