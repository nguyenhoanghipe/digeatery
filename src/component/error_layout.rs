use crate::Route;
use dioxus::{fullstack::FullstackContext, prelude::*};

#[component]
pub fn ErrorLayout() -> Element {
    rsx! {
		ErrorBoundary {
			handle_error: move |err: ErrorContext| {
			    let http_error = FullstackContext::commit_error_status(err.error().unwrap());
			    match http_error.status {
			        StatusCode::NOT_FOUND => rsx! {
				div { "404 - Page not found" }
			},
			        _ => rsx! {
				div { "An unknown error occurred" }
			},
			    }
			},
			Outlet::<Route> {}
		}
	}
}
