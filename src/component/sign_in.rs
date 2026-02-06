use dioxus::prelude::*;
use crate::lib::ui::{Button, ButtonVariant, Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label};

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
    
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Login to your account" }
                CardDescription { "Enter your email below to login to your account" }
                CardAction {
                    Button { variant: ButtonVariant::Ghost, "Sign Up" }
                }
            }
            CardContent {
                form {
                    div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                        div { style: "display: grid; gap: 0.5rem;",
                            Label { html_for: "email", "Email" }
                            Input {
                                id: "email",
                                r#type: "email",
                                placeholder: "m@example.com",
                                onchange: handle_email_change
                            }
                        }
                        div { style: "display: grid; gap: 0.5rem;",
                            div { style: "display: flex; align-items: center;",
                                Label { html_for: "password", "Password" }
                                a {
                                    href: "#",
                                    style: "margin-left: auto; font-size: 0.875rem; color: var(--secondary-color-5); text-decoration: underline; text-underline-offset: 4px;",
                                    "Forgot your password?"
                                }
                            }
                            Input { id: "password", r#type: "password", onchange: handle_password_change }
                        }
                    }
                }
            }
            CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                Button { r#type: "submit", style: "width: 100%;", onclick: submit, "Login" }
                Button { variant: ButtonVariant::Outline, style: "width: 100%;", "Login with Google" }
            }
        }
    }
}

