/*! A parser for GEDCOM files

```rust
// the parser takes the gedcom file contents as a chars iterator
use gedcom::GedcomRecord;
let gedcom_source = std::fs::read_to_string("./tests/fixtures/sample.ged").unwrap();

let mut record = GedcomRecord::new(gedcom_source.chars());
let gedcom_data = record.parse_record();

// output some stats on the gedcom contents
gedcom_data.stats();
```

This crate contains an optional `"json"` feature that implements serialization & deserialization to json with [`serde`](https://serde.rs).
*/

#![deny(clippy::pedantic)]
#![warn(missing_docs)]

#[macro_use]
mod util;
use util::{dbg, parse_custom_tag};

pub mod tokenizer;
use tokenizer::{Token, Tokenizer};

pub mod types;
use types::{
    UserDefinedData, Family, Header, Individual, MultimediaRecord, Repository, Source, Submitter,
};

mod parser;
pub use parser::Parser;

use std::str::Chars;

/// The Gedcom parser that converts the token list into a data structure
pub struct GedcomRecord<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> GedcomRecord<'a> {
    /// Creates a parser state machine for parsing a gedcom file as a chars iterator
    #[must_use]
    pub fn new(chars: Chars<'a>) -> GedcomRecord {
        let mut tokenizer = Tokenizer::new(chars);
        tokenizer.next_token();
        GedcomRecord { tokenizer }
    }

    /// Does the actual parsing of the record.
    pub fn parse_record(&mut self) -> GedcomData {
        GedcomData::new(&mut self.tokenizer, 0)
    }
}

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// The data structure representing all the data within a gedcom file
pub struct GedcomData {
    /// Header containing file metadata
    pub header: Option<Header>,
    /// List of submitters of the facts
    pub submitters: Vec<Submitter>,
    /// Individuals within the family tree
    pub individuals: Vec<Individual>,
    /// The family units of the tree, representing relationships between individuals
    pub families: Vec<Family>,
    /// A data repository where `sources` are held
    pub repositories: Vec<Repository>,
    /// Sources of facts. _ie._ book, document, census, etc.
    pub sources: Vec<Source>,
    /// A multimedia asset linked to a fact
    pub multimedia: Vec<MultimediaRecord>,
    /// Applications requiring the use of nonstandard tags should define them with a leading underscore
    /// so that they will not conflict with future GEDCOM standard tags. Systems that read
    /// user-defined tags must consider that they have meaning only with respect to a system
    /// contained in the HEAD.SOUR context.
    pub custom_data: Vec<UserDefinedData>,
}

// should maybe store these by xref if available?
impl GedcomData {
    /// contructor for GedcomData
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> GedcomData {
        let mut data = GedcomData::default();
        data.parse(tokenizer, level);
        data
    }

    /// Adds a `Family` (a relationship between individuals) to the tree
    pub fn add_family(&mut self, family: Family) {
        self.families.push(family);
    }

    /// Adds an `Individual` to the tree
    pub fn add_individual(&mut self, individual: Individual) {
        self.individuals.push(individual);
    }

    /// Adds a data `Repository` to the tree
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Adds a `Source` to the tree
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    /// Adds a `Submitter` to the tree
    pub fn add_submitter(&mut self, submitter: Submitter) {
        self.submitters.push(submitter);
    }

    /// Adds a `Multimedia` to the tree
    pub fn add_multimedia(&mut self, multimedia: MultimediaRecord) {
        self.multimedia.push(multimedia);
    }

    /// Adds a `UserDefinedData` to the tree
    pub fn add_custom_data(&mut self, data: UserDefinedData) {
        self.custom_data.push(data)
    }

    /// Outputs a summary of data contained in the tree to stdout
    pub fn stats(&self) {
        println!("----------------------");
        println!("| Gedcom Data Stats: |");
        println!("----------------------");
        println!("  submitters: {}", self.submitters.len());
        println!("  individuals: {}", self.individuals.len());
        println!("  families: {}", self.families.len());
        println!("  repositories: {}", self.repositories.len());
        println!("  sources: {}", self.sources.len());
        println!("  multimedia: {}", self.multimedia.len());
        println!("----------------------");
    }
}

impl Parser for GedcomData {
    /// Does the actual parsing of the record.
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        loop {
            // TODO is this necessary?
            let current_level = match tokenizer.current_token {
                Token::Level(n) => n,
                _ => panic!(
                    "{} Expected Level, found {:?}",
                    dbg(tokenizer),
                    tokenizer.current_token
                ),
            };

            tokenizer.next_token();

            let mut pointer: Option<String> = None;
            if let Token::Pointer(xref) = &tokenizer.current_token {
                pointer = Some(xref.to_string());
                tokenizer.next_token();
            }

            if let Token::Tag(tag) = &tokenizer.current_token {
                match tag.as_str() {
                    "HEAD" => self.header = Some(Header::new(tokenizer, level)),
                    "FAM" => self.add_family(Family::new(tokenizer, level, pointer)),
                    "INDI" => {
                        self.add_individual(Individual::new(tokenizer, current_level, pointer))
                    }
                    "REPO" => {
                        self.add_repository(Repository::new(tokenizer, current_level, pointer))
                    }
                    "SOUR" => self.add_source(Source::new(tokenizer, current_level, pointer)),
                    "SUBM" => self.add_submitter(Submitter::new(tokenizer, level, pointer)),
                    "OBJE" => self.add_multimedia(MultimediaRecord::new(tokenizer, level, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled tag {}", dbg(tokenizer), tag);
                        tokenizer.next_token();
                    }
                };
            } else if let Token::CustomTag(tag) = &tokenizer.current_token {
                let tag_clone = tag.clone();
                self.add_custom_data(parse_custom_tag(tokenizer, tag_clone));
                while tokenizer.current_token != Token::Level(level) {
                    tokenizer.next_token();
                }
            } else {
                println!(
                    "{} Unhandled token {:?}",
                    dbg(tokenizer),
                    tokenizer.current_token
                );
                tokenizer.next_token();
            };
        }
    }
}

#[must_use]
/// Helper function for converting GEDCOM file content stream to parsed data.
pub fn parse(content: std::str::Chars) -> GedcomData {
    let mut p = GedcomRecord::new(content);
    p.parse_record()
}
