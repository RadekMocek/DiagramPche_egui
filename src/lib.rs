#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::App;

mod config;
mod gui;
mod helper;
mod logic;
mod model;
mod style;
