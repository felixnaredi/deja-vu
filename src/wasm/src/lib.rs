#[macro_use]
extern crate derive_builder;

mod coder;
mod game;
mod history;
mod rng;
mod web_api;

pub use web_api::{
  Commit,
  Encoded,
  Game,
  History,
};
