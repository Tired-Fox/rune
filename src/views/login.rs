use std::sync::Arc;

use dioxus::prelude::*;
use manrex::Client;
use tokio::sync::Mutex;

#[component]
pub fn Login() -> Element {
    let client = use_context::<Arc<Mutex<Client>>>();

    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        label { "Username" }
        input {
            r#type: "text",
            value: "{username}",
            oninput: move |evt| username.set(evt.value())
        }
        label { "Password" }
        input {
            r#type: "password",
            value: "{password}",
            oninput: move |evt| password.set(evt.value())
        }
        button {
            onclick: move |_evt| {
                let client = client.clone();
                async move {
                    client
                        .lock()
                        .await
                        .oauth_mut()
                        .login_with(username.to_string(), password.to_string())
                        .await?;
                    Ok(())
                }
            },
            "Login"
        }
    }
}
