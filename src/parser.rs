//! The state machine that parses a char iterator of the gedcom's contents
use std::{panic, str::Chars};

use crate::tokenizer::{Token, Tokenizer};
use crate::tree::GedcomData;
use crate::types::{Family, Header, Individual, Repository, Source, Submitter};
use crate::util::{dbg, parse_custom_tag};

/// Parse converts a subset of a token list into a type's data structure.
pub trait Parse {
    /// parse does the actual parsing of a subset of a token list
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8);
}

/// The Gedcom parser that converts the token list into a data structure
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a parser state machine for parsing a gedcom file as a chars iterator
    #[must_use]
    pub fn new(chars: Chars<'a>) -> Parser {
        let mut tokenizer = Tokenizer::new(chars);
        tokenizer.next_token();
        Parser { tokenizer }
    }

    /// Does the actual parsing of the record.
    pub fn parse_record(&mut self) -> GedcomData {
        let mut data = GedcomData::default();
        loop {
            let level = match self.tokenizer.current_token {
                Token::Level(n) => n,
                _ => panic!(
                    "{} Expected Level, found {:?}",
                    dbg(&self.tokenizer),
                    self.tokenizer.current_token
                ),
            };

            self.tokenizer.next_token();

            let mut pointer: Option<String> = None;
            if let Token::Pointer(xref) = &self.tokenizer.current_token {
                pointer = Some(xref.to_string());
                self.tokenizer.next_token();
            }

            if let Token::Tag(tag) = &self.tokenizer.current_token {
                match tag.as_str() {
                    "HEAD" => data.header = Some(Header::new(&mut self.tokenizer, 0)),
                    "FAM" => data.add_family(Family::new(&mut self.tokenizer, 0, pointer)),
                    "INDI" => {
                        data.add_individual(Individual::new(&mut self.tokenizer, level, pointer))
                    }
                    "REPO" => data.add_repository(Repository::new(&mut self.tokenizer, level, pointer)),
                    "SOUR" => data.add_source(Source::new(&mut self.tokenizer, level, pointer)),
                    "SUBM" => data.add_submitter(Submitter::new(&mut self.tokenizer, 0, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled tag {}", dbg(&self.tokenizer), tag);
                        self.tokenizer.next_token();
                    }
                };
            } else if let Token::CustomTag(tag) = &self.tokenizer.current_token {
                // TODO
                let tag_clone = tag.clone();
                let custom_data = parse_custom_tag(&mut self.tokenizer, tag_clone);
                println!(
                    "{} Skipping top-level custom tag: {:?}",
                    dbg(&self.tokenizer),
                    custom_data
                );
                while self.tokenizer.current_token != Token::Level(0) {
                    self.tokenizer.next_token();
                }
            } else {
                println!(
                    "{} Unhandled token {:?}",
                    dbg(&self.tokenizer),
                    self.tokenizer.current_token
                );
                self.tokenizer.next_token();
            };
        }
        data
    }
}
