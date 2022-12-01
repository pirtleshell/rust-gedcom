/*! A parser for GEDCOM files

```rust
// the parser takes the gedcom file contents as a chars iterator
use gedcom::GedcomDocument;
let gedcom_source = std::fs::read_to_string("./tests/fixtures/sample.ged").unwrap();

let mut doc = GedcomDocument::new(gedcom_source.chars());
let gedcom_data = doc.parse_document();

// output some stats on the gedcom contents
gedcom_data.stats();
```

This crate contains an optional `"json"` feature that implements serialization & deserialization to json with [`serde`](https://serde.rs).
*/

#![deny(clippy::pedantic)]
#![warn(missing_docs)]

use std::str::Chars;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[macro_use]
mod util;

pub mod tokenizer;
use tokenizer::{Token, Tokenizer};

pub mod types;
use types::{
    Family, Header, Individual, MultimediaRecord, Repository, Source, Submission, Submitter,
    UserDefinedData,
};

/// The GedcomDocument can convert the token list into a data structure. The order of the Dataset
/// should be as follows: the HEAD must come first and TRLR must be last, with any RECORDs in
/// between.
///
/// # A Minimal Example
///
/// ```rust
/// use gedcom::GedcomDocument;
/// let sample = "\
///    0 HEAD\n\
///    1 GEDC\n\
///    2 VERS 5.5\n\
///    0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
///
/// let head = data.header.unwrap();
/// let gedc = head.gedcom.unwrap();
/// assert_eq!(gedc.version.unwrap(), "5.5");
/// ```
pub struct GedcomDocument<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> GedcomDocument<'a> {
    /// Creates a parser state machine for parsing a gedcom file as a chars iterator
    #[must_use]
    pub fn new(chars: Chars<'a>) -> GedcomDocument {
        let mut tokenizer = Tokenizer::new(chars);
        tokenizer.next_token();
        GedcomDocument { tokenizer }
    }

    /// Does the actual parsing of the record.
    pub fn parse_document(&mut self) -> GedcomData {
        GedcomData::new(&mut self.tokenizer, 0)
    }
}

/// The Parser trait converts a subset of a token list into a type's data structure.
pub trait Parser {
    /// parse does the actual parsing of a subset of a token list
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8);
}

#[must_use]
/// Helper function for converting GEDCOM file content stream to parsed data.
pub fn parse_ged(content: std::str::Chars) -> GedcomData {
    let mut p = GedcomDocument::new(content);
    p.parse_document()
}

/// parse_subset is a helper function that handles some boilerplate code involved in implementing
/// the Parser trait. It returns a Vector of any UserDefinedData.
pub fn parse_subset<F>(
    tokenizer: &mut Tokenizer,
    level: u8,
    mut tag_handler: F,
) -> Vec<UserDefinedData>
where
    F: FnMut(&str, &mut Tokenizer),
{
    let mut custom_data = Vec::new();
    loop {
        if let Token::Level(curl_level) = tokenizer.current_token {
            if curl_level <= level {
                break;
            }
        }

        match &tokenizer.current_token {
            Token::Tag(tag) => {
                let tag_clone = tag.clone();
                tag_handler(tag_clone.as_str(), tokenizer);
            }
            Token::CustomTag(tag) => {
                let tag_clone = tag.clone();
                custom_data.push(parse_custom_tag(tokenizer, tag_clone));
            }
            Token::Level(_) => tokenizer.next_token(),
            _ => panic!(
                "{}, Unhandled Token: {:?}",
                tokenizer.debug(),
                tokenizer.current_token
            ),
        }
    }
    custom_data
}

/// parse_custom_tag handles User Defined Data. See Gedcom 5.5 spec, p.56
pub fn parse_custom_tag(tokenizer: &mut Tokenizer, tag: String) -> UserDefinedData {
    let value = tokenizer.take_line_value();
    UserDefinedData { tag, value }
}

/// GedcomData is the data structure representing all the data within a gedcom file
///
/// # Example
///
/// ```rust
/// use gedcom::GedcomDocument;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     0 @SUBMITTER@ SUBM\n\
///     0 @PERSON1@ INDI\n\
///     0 @FAMILY1@ FAM\n\
///     0 @R1@ REPO\n\
///     0 @SOURCE1@ SOUR\n\
///     0 @MEDIA1@ OBJE\n\
///     0 _MYOWNTAG This is a non-standard tag. Not recommended but allowed\n\
///     0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
///
/// assert_eq!(data.submitters.len(), 1);
/// assert_eq!(data.submitters[0].xref.as_ref().unwrap(), "@SUBMITTER@");
///
/// assert_eq!(data.individuals.len(), 1);
/// assert_eq!(data.individuals[0].xref.as_ref().unwrap(), "@PERSON1@");
///
/// assert_eq!(data.families.len(), 1);
/// assert_eq!(data.families[0].xref.as_ref().unwrap(), "@FAMILY1@");
///
/// assert_eq!(data.repositories.len(), 1);
/// assert_eq!(data.repositories[0].xref.as_ref().unwrap(), "@R1@");
///
/// assert_eq!(data.sources.len(), 1);
/// assert_eq!(data.sources[0].xref.as_ref().unwrap(), "@SOURCE1@");
///
/// assert_eq!(data.custom_data.len(), 1);
/// assert_eq!(data.custom_data[0].tag, "_MYOWNTAG");
/// assert_eq!(data.custom_data[0].value, "This is a non-standard tag. Not recommended but allowed");
/// ```
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct GedcomData {
    /// Header containing file metadata
    pub header: Option<Header>,
    /// List of submitters of the facts
    pub submitters: Vec<Submitter>,
    /// List of submission records
    pub submissions: Vec<Submission>,
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

    /// Add a `Submission` to the tree
    pub fn add_submission(&mut self, submission: Submission) {
        self.submissions.push(submission);
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
        println!("  submissions: {}", self.submissions.len());
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
            let current_level = match tokenizer.current_token {
                Token::Level(n) => n,
                _ => panic!(
                    "{} Expected Level, found {:?}",
                    tokenizer.debug(),
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
                    "SUBN" => self.add_submission(Submission::new(tokenizer, level, pointer)),
                    "SUBM" => self.add_submitter(Submitter::new(tokenizer, level, pointer)),
                    "OBJE" => self.add_multimedia(MultimediaRecord::new(tokenizer, level, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled tag {}", tokenizer.debug(), tag);
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
                    tokenizer.debug(),
                    tokenizer.current_token
                );
                tokenizer.next_token();
            };
        }
    }
}
