use dioxus::logger::tracing;
use dioxus::prelude::*;

static CSS: Asset = asset!("./sign_in.css");

#[component]
pub fn SignIn() -> Element {
    
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    
    let handle_email_change = move |evt: Event<FormData>| {
        email.set(evt.data.value());
        debug!("{}", email());
    };
   
    let handle_password_change = move |evt: Event<FormData>| {
        password.set(evt.data.value());
        debug!("{}", password());
    };
    
    let submit = move |evt| {
        println!("{:?}", evt);
    };
    
    rsx!{
        Stylesheet { href: CSS }
        div {
            class: "sign-in",
            
            label {"Email"}
            input { type:  "text", onchange: handle_email_change, placeholder : "Enter your email"}
            label {"Password"}
            input { type:  "password", onchange: handle_password_change }
            button { onclick: submit, "Sign In" }
        }
    }
}

