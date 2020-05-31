#![deny(clippy::pedantic)]

#[macro_use]
mod util;

pub mod tokenizer;
pub mod types;
pub mod parser;

mod tree;
pub use tree::GedcomData;
