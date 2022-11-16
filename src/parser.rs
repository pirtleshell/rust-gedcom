//! The state machine that parses a char iterator of the gedcom's contents
use std::str::Chars;
use crate::tokenizer::Tokenizer;
use crate::tree::GedcomData;

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
        GedcomData::new(&mut self.tokenizer, 0)
    }
}
