#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

pub mod app;
pub mod fragments;
pub mod error;
pub mod routes;
pub mod services;
pub mod types;
pub mod gqls;
// pub mod utils;

// use wasm_bindgen::prelude::*;

use app::App;

// Use `wee_alloc` as the global allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
