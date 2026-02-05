use crate::template::{Echo, Hero};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Template() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
