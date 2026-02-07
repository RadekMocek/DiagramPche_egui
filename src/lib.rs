#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::App;

mod config;
mod gui;
mod logic;
mod model;
mod style;
