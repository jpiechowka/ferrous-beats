#![allow(non_snake_case)]

mod pages {
    pub(crate) mod converter;
    pub(crate) mod downloader;
    pub(crate) mod music_library;
}
mod components {
    pub(crate) mod footer;
    pub(crate) mod navbar;
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
