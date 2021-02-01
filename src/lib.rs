/*! A parser for GEDCOM files

```rust
use gedcom::parser::Parser;

// the parser takes the gedcom file contents as a chars iterator
let gedcom_source = std::fs::read_to_string("./tests/fixtures/sample.ged").unwrap();

let mut parser = Parser::new(gedcom_source.chars());
let gedcom_data = parser.parse_record();

// output some stats on the gedcom contents
gedcom_data.stats();
```
*/

#![deny(clippy::pedantic)]
#![warn(missing_docs)]

#[macro_use]
mod util;

pub mod parser;
pub mod tokenizer;
pub mod types;

mod tree;
pub use tree::GedcomData;
