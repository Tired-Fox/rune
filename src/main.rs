use std::sync::Arc;

use dioxus::prelude::*;

mod layouts;
mod components;
mod views;

use manrex::{auth::{Credentials, OAuth}, Client};
use tokio::sync::Mutex;
use views::{Login, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(layouts::Default)]
    #[route("/")]
    Home {},
    #[layout(layouts::Default)]
    #[route("/login")]
    Login {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let auth = OAuth::new(Credentials::from_env()?);
    use_context_provider(move || Arc::new(Mutex::new(Client::new(auth))));

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
