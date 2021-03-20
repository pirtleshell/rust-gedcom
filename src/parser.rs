//! The state machine that parses a char iterator of the gedcom's contents
use std::{error::Error, fmt, panic, str::Chars};

use crate::tokenizer::{Token, Tokenizer};
use crate::tree::Gedcom;
use crate::types::{
    event::HasEvents, Address, CustomData, Event, Family, FamilyLink, Gender, Header, Individual,
    Name, RepoCitation, Repository, Source, SourceCitation, Submitter,
};

/// The Gedcom parser that converts the token list into a data structure
pub struct Parser<'a> {
    pub(crate) tokenizer: Tokenizer<'a>,
}

// TODO: expose useful helpers without publicizing tokenizer

impl<'a> Parser<'a> {
    /// Creates a parser state machine for parsing a gedcom file as a chars iterator
    #[must_use]
    pub fn new(chars: Chars<'a>) -> Parser {
        let mut tokenizer = Tokenizer::new(chars);
        tokenizer.next_token();
        Parser { tokenizer }
    }

    /// Does the actual parsing of the record.
    pub fn parse_record(&mut self) -> Gedcom {
        let mut data = Gedcom::default();
        loop {
            let level = match self.tokenizer.current_token {
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
                    "HEAD" => data.header = Header::parse(self, 0).unwrap(),
                    "FAM" => {
                        data.add_family(Family::parse(self, level).unwrap().with_xref(pointer))
                    }
                    "INDI" => data.add_individual(self.parse_individual(level, pointer)),
                    "REPO" => data.add_repository(self.parse_repository(level, pointer)),
                    "SOUR" => data.add_source(self.parse_source(level, pointer)),
                    "SUBM" => data.add_submitter(self.parse_submitter(level, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled top-level data {}", self.dbg(), tag);
                        self.skip_block(level)
                    }
                };
            } else if let Token::CustomTag(tag) = &self.tokenizer.current_token {
                // TODO
                let tag_clone = tag.clone();
                let custom_data = self.parse_custom_tag(tag_clone);
                println!(
                    "{} Skipping top-level custom tag: {:?}",
                    self.dbg(),
                    custom_data
                );
                self.skip_block(level);
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
    fn parse_submitter(&mut self, level: u8, xref: Option<String>) -> Submitter {
        // skip over SUBM tag name
        self.tokenizer.next_token();

        let mut submitter = Submitter::new(xref);
        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => submitter.name = Some(self.take_line_value()),
                    "ADDR" => {
                        submitter.address = Some(self.parse_address(level + 1));
                    }
                    "PHON" => submitter.phone = Some(self.take_line_value()),
                    _ => self.skip_current_tag(level + 1, "Submitter"),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => self.handle_unexpected_token(level + 1, "SUBM"),
            }
        }
        submitter
    }

    /// Parses INDI top-level tag
    fn parse_individual(&mut self, level: u8, xref: Option<String>) -> Individual {
        // skip over INDI tag name
        self.tokenizer.next_token();
        let mut individual = Individual::new(xref);

        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => individual.name = Some(self.parse_name(level + 1)),
                    "SEX" => individual.sex = self.parse_gender(),
                    "ADOP" | "BIRT" | "BAPM" | "BARM" | "BASM" | "BLES" | "BURI" | "CENS"
                    | "CHR" | "CHRA" | "CONF" | "CREM" | "DEAT" | "EMIG" | "FCOM" | "GRAD"
                    | "IMMI" | "NATU" | "ORDN" | "RETI" | "RESI" | "PROB" | "WILL" | "EVEN" => {
                        individual.add_event(self.parse_event(level + 1));
                    }
                    "FAMC" | "FAMS" => {
                        let tag_clone = tag.clone();
                        individual
                            .add_family(self.parse_family_link(tag_clone.as_str(), level + 1));
                    }
                    "CHAN" => {
                        // assuming it always only has a single DATE subtag
                        self.tokenizer.next_token(); // level
                        self.tokenizer.next_token(); // DATE tag
                        individual.last_updated = Some(self.take_line_value());
                    }
                    _ => self.skip_current_tag(level + 1, "Individual"),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    individual.add_custom_data(self.parse_custom_tag(tag_clone))
                }
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled Individual Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }

        individual
    }

    fn parse_source(&mut self, level: u8, xref: Option<String>) -> Source {
        // skip SOUR tag
        self.tokenizer.next_token();
        let mut source = Source::new(xref);

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATA" => self.tokenizer.next_token(),
                    // TODO: cleanup to just use parse_event
                    // "EVEN" => {
                    //     let events_recorded = self.take_line_value();
                    //     let mut event = self.parse_event(level + 2);
                    //     event.with_source_data(events_recorded);
                    //     source.data.add_event(event);
                    // }
                    "AGNC" => source.data.agency = Some(self.take_line_value()),
                    "ABBR" => source.abbreviation = Some(self.take_continued_text(level + 1)),
                    "TITL" => source.title = Some(self.take_continued_text(level + 1)),
                    "REPO" => source.add_repo_citation(self.parse_repo_citation(level + 1)),
                    _ => self.skip_current_tag(level + 1, "Source"),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => self.handle_unexpected_token(level + 1, "SOUR"),
            }
        }

        source
    }

    /// Parses REPO top-level tag.
    fn parse_repository(&mut self, level: u8, xref: Option<String>) -> Repository {
        // skip REPO tag
        self.tokenizer.next_token();
        let mut repo = Repository {
            xref,
            name: None,
            address: None,
        };
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => repo.name = Some(self.take_line_value()),
                    "ADDR" => repo.address = Some(self.parse_address(level + 1)),
                    _ => self.skip_current_tag(level + 1, "Repository"),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => self.handle_unexpected_token(level + 1, "REPO"),
            }
        }

        repo
    }

    fn parse_custom_tag(&mut self, tag: String) -> CustomData {
        let value = self.take_line_value();
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
                Token::Level(_) => self.tokenizer.next_token(),
                _ => self.handle_unexpected_token(2, "GEDC"),
            }
        }
        header
    }

    fn parse_family_link(&mut self, tag: &str, level: u8) -> FamilyLink {
        let xref = self.take_line_value();
        let mut link = FamilyLink::new(xref, tag);

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PEDI" => link.set_pedigree(self.take_line_value().as_str()),
                    _ => self.skip_current_tag(level + 1, "FamilyLink"),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => self.handle_unexpected_token(level + 1, "FamilyLink"),
            }
        }

        link
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
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled RepoCitation Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }
        citation
    }

    fn parse_gender(&mut self) -> Gender {
        self.tokenizer.next_token();
        let gender: Gender;
        if let Token::LineValue(gender_string) = &self.tokenizer.current_token {
            gender = match gender_string.as_str() {
                "M" => Gender::Male,
                "F" => Gender::Female,
                "N" => Gender::Nonbinary,
                "U" => Gender::Unknown,
                _ => panic!("{} Unknown gender value {}", self.dbg(), gender_string),
            };
        } else {
            panic!(
                "Expected gender LineValue, found {:?}",
                self.tokenizer.current_token
            );
        }
        self.tokenizer.next_token();
        gender
    }

    fn parse_name(&mut self, level: u8) -> Name {
        let mut name = Name::default();
        name.value = Some(self.take_line_value());
        let mut cur_level = level;

        loop {
            if let Token::Level(new_level) = self.tokenizer.current_token {
                if new_level <= cur_level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "GIVN" => name.given = Some(self.take_line_value()),
                    "NPFX" => name.prefix = Some(self.take_line_value()),
                    "NSFX" => name.suffix = Some(self.take_line_value()),
                    "SPFX" => name.surname_prefix = Some(self.take_line_value()),
                    "SURN" => name.surname = Some(self.take_line_value()),
                    _ => self.skip_current_tag(cur_level, "Name"),
                },
                Token::Level(_) => {
                    cur_level += 1;
                    self.tokenizer.next_token()
                }
                _ => panic!("Unhandled Name Token: {:?}", self.tokenizer.current_token),
            }
        }

        name
    }

    fn parse_event(&mut self, level: u8) -> Event {
        match Event::parse(self, level) {
            Ok(event) => event,
            Err(e) => panic!("event parsing fail: {:?}", e),
        }
    }

    /// Parses ADDR tag
    fn parse_address(&mut self, level: u8) -> Address {
        match Address::parse(self, level) {
            Ok(addr) => addr,
            Err(e) => panic!("address fail: {:?}", e),
        }
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
                Token::Level(_) => self.tokenizer.next_token(),
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
                    "CONC" => {
                        value.push(' ');
                        value.push_str(&self.take_line_value())
                    }
                    _ => panic!("{} Unhandled Continuation Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
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

    pub(crate) fn skip_current_tag(&mut self, level: u8, parent_name: &str) {
        if let Token::Tag(tag) = &self.tokenizer.current_token {
            println!(
                "{} Unhandled {} Tag: {}",
                self.dbg_lvl(level),
                parent_name,
                tag
            );
        } else {
            panic!("Expected tag, found {:?}", &self.tokenizer.current_token);
        }
        self.skip_block(level);
    }

    pub(crate) fn handle_unexpected_token(&mut self, level: u8, base_tag: &str) {
        println!(
            "{} Unhandled {} Token: {:?}",
            self.dbg_lvl(level),
            base_tag,
            &self.tokenizer.current_token
        );
        self.skip_block(level);
    }

    pub(crate) fn skip_block(&mut self, level: u8) {
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            self.tokenizer.next_token();
        }
    }

    fn dbg_lvl(&self, level: u8) -> String {
        format!("line {}, level {}:", self.tokenizer.line, level)
    }

    /// Debug function displaying GEDCOM line number of error message.
    pub(crate) fn dbg(&self) -> String {
        format!("line {}:", self.tokenizer.line)
    }
}

/// Trait given to data types that can be parsed into `GedcomData`
pub trait Parsable<T> {
    /// Parses an object by iterating through the `parser` until no longer at given
    /// `level` or deeper.
    ///
    /// # Errors
    /// Raises a `ParsingError` when unhandled or unexpected tokens are found.
    fn parse(parser: &mut Parser, level: u8) -> Result<T, ParsingError>;
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
