use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Default() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-[100vh] w-full",
            div {
                class: "h-full overflow-auto w-full",
                Outlet::<Route> {}
            }
            div {
                class: "w-full h-6 bg-zinc-900 text-white text-center",
                "Built with \u{2661} using "
                a {
                    class: "underline",
                    href: "https://mangadex.org/",
                    target: "_blank",
                    "MangaDex"
                }
            }
        }
    }
}
