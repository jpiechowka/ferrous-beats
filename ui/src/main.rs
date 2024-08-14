#![allow(non_snake_case)]

mod pages {
    pub mod converter;
    pub mod downloader;
    pub mod music_library;
}
mod components {
    pub mod footer;
    pub mod navbar;
}

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use crate::pages::music_library::MusicLibrary;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    MusicLibrary {},
    // #[route("/downloader")]
    // Downloader {},
    // #[route("/converter")]
    // Converter {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
