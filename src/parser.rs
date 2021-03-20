//! The state machine that parses a char iterator of the gedcom's contents
use std::{error::Error, fmt, panic, str::Chars};

use crate::tokenizer::{Token, Tokenizer};
use crate::tree::Gedcom;
use crate::types::{
    Address, CustomData, Event, Family, Header, Individual, RepoCitation, Repository, Source,
    SourceCitation, Submitter,
};

/// The Gedcom parser that converts the token list into a data structure
pub struct Parser<'a> {
    pub(crate) tokenizer: Tokenizer<'a>,
    pub(crate) level: u8,
}

// TODO: expose useful helpers without publicizing tokenizer

impl<'a> Parser<'a> {
    /// Creates a parser state machine for parsing a gedcom file as a chars iterator
    #[must_use]
    pub fn new(chars: Chars<'a>) -> Parser {
        let mut tokenizer = Tokenizer::new(chars);
        if tokenizer.current_token == Token::None {
            tokenizer.next_token();
            Parser {
                tokenizer,
                level: 0,
            }
        } else {
            panic!(
                "Unexpected starting token, found {:?}",
                &tokenizer.current_token
            );
        }
    }

    pub(crate) fn set_level(&mut self) {
        if let Token::Level(lvl) = self.tokenizer.current_token {
            self.level = lvl;
            self.tokenizer.next_token();
        } else {
            panic!(
                "{} Expected Level, found {:?}",
                self.dbg(),
                &self.tokenizer.current_token
            );
        }
    }

    /// Does the actual parsing of the record.
    pub fn parse_record(&mut self) -> Gedcom {
        let mut data = Gedcom::default();
        loop {
            self.level = match self.tokenizer.current_token {
                Token::Level(n) => n,
                _ => panic!(
                    "{} Expected Level, found {:?}",
                    self.dbg(),
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
                    "HEAD" => data.header = Header::parse(self).unwrap(),
                    "FAM" => data.add_family(Family::parse(self).unwrap().with_xref(pointer)),
                    "INDI" => {
                        let mut individual = Individual::parse(self).unwrap();
                        individual.xref = pointer;
                        data.add_individual(individual);
                    }
                    "REPO" => data.add_repository(self.parse_repository(pointer)),
                    "SOUR" => data.add_source(self.parse_source(pointer)),
                    "SUBM" => data.add_submitter(self.parse_submitter(pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled top-level data {}", self.dbg(), tag);
                        self.skip_block()
                    }
                };
            } else if let Token::CustomTag(_) = &self.tokenizer.current_token {
                let custom_data = self.parse_custom_tag();
                println!(
                    "{} Skipping top-level custom tag: {:?}",
                    self.dbg(),
                    custom_data
                );
                self.skip_block();
            } else {
                println!(
                    "{} Unhandled token {:?}",
                    self.dbg(),
                    self.tokenizer.current_token
                );
                self.tokenizer.next_token();
            };
        }

        data
    }

    /// Parses SUBM top-level tag
    fn parse_submitter(&mut self, xref: Option<String>) -> Submitter {
        let base_lvl = self.level;
        // skip over SUBM tag name
        self.tokenizer.next_token();

        let mut submitter = Submitter::new(xref);
        while self.tokenizer.current_token != Token::Level(base_lvl) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => submitter.name = Some(self.take_line_value()),
                    "ADDR" => {
                        submitter.address = Some(Address::parse(self).unwrap());
                    }
                    "PHON" => submitter.phone = Some(self.take_line_value()),
                    _ => self.skip_current_tag(self.level, "Submitter"),
                },
                Token::Level(_) => self.set_level(),
                _ => self.handle_unexpected_token(self.level, "SUBM"),
            }
        }
        submitter
    }

    fn parse_source(&mut self, xref: Option<String>) -> Source {
        let base_lvl = self.level;
        // skip SOUR tag
        self.tokenizer.next_token();
        let mut source = Source::new(xref);

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= base_lvl {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATA" => self.tokenizer.next_token(),
                    // TODO: cleanup to just use parse_event
                    "EVEN" => source.data.add_event(Event::parse(self).unwrap()),
                    "AGNC" => source.data.agency = Some(self.take_line_value()),
                    "ABBR" => source.abbreviation = Some(self.take_continued_text(self.level)),
                    "TITL" => source.title = Some(self.take_continued_text(self.level)),
                    "REPO" => source.add_repo_citation(self.parse_repo_citation(self.level)),
                    _ => self.skip_current_tag(self.level, "Source"),
                },
                Token::Level(_) => self.set_level(),
                _ => self.handle_unexpected_token(self.level, "SOUR"),
            }
        }

        source
    }

    /// Parses REPO top-level tag.
    fn parse_repository(&mut self, xref: Option<String>) -> Repository {
        let base_lvl = self.level;
        // skip REPO tag
        self.tokenizer.next_token();
        let mut repo = Repository {
            xref,
            name: None,
            address: None,
        };
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= base_lvl {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => repo.name = Some(self.take_line_value()),
                    "ADDR" => repo.address = Some(Address::parse(self).unwrap()),
                    _ => self.skip_current_tag(self.level, "Repository"),
                },
                Token::Level(_) => self.set_level(),
                _ => self.handle_unexpected_token(self.level, "REPO"),
            }
        }

        repo
    }

    pub(crate) fn parse_custom_tag(&mut self) -> CustomData {
        let tag: String = self.take_tag().into();
        let value: String = self.take_line_value();
        CustomData { tag, value }
    }

    /// Handle parsing GEDC tag
    pub(crate) fn parse_gedcom_data(&mut self, mut header: Header) -> Header {
        // skip GEDC tag
        self.tokenizer.next_token();

        while self.tokenizer.current_token != Token::Level(1) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "VERS" => header.gedcom_version = Some(self.take_line_value()),
                    // this is the only value that makes sense. warn them otherwise.
                    "FORM" => {
                        let form = self.take_line_value();
                        if &form.to_uppercase() != "LINEAGE-LINKED" {
                            println!(
                                "WARNING: Unrecognized GEDCOM form. Expected LINEAGE-LINKED, found {}"
                            , form);
                        }
                    }
                    _ => panic!("{} Unhandled GEDC Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.set_level(),
                _ => self.handle_unexpected_token(2, "GEDC"),
            }
        }
        header
    }

    fn parse_repo_citation(&mut self, level: u8) -> RepoCitation {
        let xref = self.take_line_value();
        let mut citation = RepoCitation {
            xref,
            call_number: None,
        };
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CALN" => citation.call_number = Some(self.take_line_value()),
                    _ => panic!("{} Unhandled RepoCitation Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.set_level(),
                _ => panic!(
                    "Unhandled RepoCitation Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }
        citation
    }

    // TODO Citation::parse
    pub(crate) fn parse_citation(&mut self, level: u8) -> SourceCitation {
        let mut citation = SourceCitation {
            xref: self.take_line_value(),
            page: None,
        };
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PAGE" => citation.page = Some(self.take_line_value()),
                    _ => self.skip_current_tag(level + 1, "Citation"),
                },
                Token::Level(_) => self.set_level(),
                _ => self.handle_unexpected_token(level + 1, "Citation"),
            }
        }
        citation
    }

    /// Takes the value of the current line including handling
    /// multi-line values from CONT & CONC tags.
    pub(crate) fn take_continued_text(&mut self, level: u8) -> String {
        let mut value = self.take_line_value();

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CONT" => {
                        value.push('\n');
                        value.push_str(&self.take_line_value())
                    }
                    "CONC" => value.push_str(&self.take_line_value()),
                    _ => panic!("{} Unhandled Continuation Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.set_level(),
                _ => panic!(
                    "Unhandled Continuation Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }

        value
    }

    /// Grabs and returns to the end of the current line as a String
    pub(crate) fn take_line_value(&mut self) -> String {
        let value: String;
        self.tokenizer.next_token();

        if let Token::LineValue(val) = &self.tokenizer.current_token {
            value = val.to_string();
        } else {
            panic!(
                "{} Expected LineValue, found {:?}",
                self.dbg(),
                self.tokenizer.current_token
            );
        }
        self.tokenizer.next_token();
        value
    }

    pub(crate) fn take_tag(&mut self) -> &str {
        match &self.tokenizer.current_token {
            Token::Tag(tag) | Token::CustomTag(tag) => tag,
            _ => panic!("Expected tag, found {:?}", &self.tokenizer.current_token),
        }
    }

    pub(crate) fn skip_current_tag(&mut self, level: u8, parent_name: &str) {
        let dbg = self.dbg();
        let tag = self.take_tag();
        println!("{} Unhandled {} Tag: {}", dbg, parent_name, tag);
        self.skip_block();
    }

    pub(crate) fn handle_unexpected_token(&mut self, level: u8, base_tag: &str) {
        println!(
            "{} Unhandled {} Token: {:?}",
            self.dbg(),
            base_tag,
            &self.tokenizer.current_token
        );
        self.skip_block();
    }

    pub(crate) fn skip_block(&mut self) {
        let block_level = self.level;
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= block_level {
                    break;
                }
            }
            self.tokenizer.next_token();
        }
    }

    /// Debug function displaying GEDCOM line number of error message.
    pub(crate) fn dbg(&self) -> String {
        format!("line {}, level {} :", &self.tokenizer.line, &self.level)
    }
}

/// Trait given to data types that can be parsed into `GedcomData`
pub trait Parsable<T> {
    /// Parses an object by iterating through the `parser` until no longer at given
    /// `level` or deeper.
    ///
    /// # Errors
    /// Raises a `ParsingError` when unhandled or unexpected tokens are found.
    fn parse(parser: &mut Parser) -> Result<T, ParsingError>;
}

#[derive(Debug)]
/// Error indicating unhandled or unexpected token encountered.
pub struct ParsingError {
    line: usize,
    token: Token,
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("line: {}\n{:?}", self.line, self.token))
    }
}
