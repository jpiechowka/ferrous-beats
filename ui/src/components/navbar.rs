use dioxus::prelude::*;

#[component]
pub(crate) fn Navbar() -> Element {
    rsx! {
        nav { class: "bg-amber-800",
            div {class: "mx-auto px-4 py-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center",
                    div { class: "flex space-x-4",
                        a { class: "text-white hover:text-amber-200 text-lg", href: "#library", "Library" }
                        a { class: "text-white hover:text-amber-200 text-lg", href: "#downloader", "Downloader" }
                        a { class: "text-white hover:text-amber-200 text-lg", href: "#converter", "Converter" }
                    }
                    div { class: "flex-grow" }
                    div { class: "relative hidden md:block",
                        input {
                            class: "bg-amber-600 text-zinc-100 placeholder-amber-200 rounded-xl py-2 px-4 text-lg focus:outline-none focus:ring-2 focus:ring-amber-300",
                            placeholder: "Search..."
                        }
                    }
                }
            }
        }
    }
}
