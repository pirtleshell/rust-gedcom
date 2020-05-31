#![deny(clippy::pedantic)]

#[macro_use]
mod util;

pub mod parser;
pub mod tokenizer;
pub mod types;

mod tree;
pub use tree::GedcomData;
