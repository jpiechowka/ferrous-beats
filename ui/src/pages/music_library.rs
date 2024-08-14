use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use dioxus::prelude::*;

#[component]
pub fn MusicLibrary() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}
            main { class: "flex-grow bg-zinc-900", // Dark background
                // Your main content goes here
                div { class: "container mx-auto px-4 py-8",
                    // Example content
                    h1 { class: "text-3xl font-bold text-zinc-100", "Welcome to Ferrous Beats" }
                    p { class: "mt-4 text-zinc-200", "Enjoy your music library and more!" }
                }
            }
            Footer {}
        }
    }
}
