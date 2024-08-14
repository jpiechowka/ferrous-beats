use dioxus::prelude::*;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
          footer { class: "bg-amber-800 text-stone-100",
            div { class: "mx-auto px-4 py-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center",
                    p { class: "text-xl font-bold", "Ferrous Beats" }
                    p { class: "font-medium", "Version: {APP_VERSION}" }
                }
            }
        }
    }
}
