//! The state machine that parses a char iterator of the gedcom's contents
use std::{panic, str::Chars};

use crate::tokenizer::{Token, Tokenizer};
use crate::tree::GedcomData;
use crate::types::{
    Address, CustomData, Event, Family, FamilyLink, Gender, Header, Individual, Name, RepoCitation,
    Repository, Source, SourceCitation, Submitter,
};

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
                    "HEAD" => data.header = self.parse_header(),
                    "FAM" => data.add_family(self.parse_family(level, pointer)),
                    "INDI" => data.add_individual(self.parse_individual(level, pointer)),
                    "REPO" => data.add_repository(self.parse_repository(level, pointer)),
                    "SOUR" => data.add_source(self.parse_source(level, pointer)),
                    "SUBM" => data.add_submitter(self.parse_submitter(level, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled tag {}", self.dbg(), tag);
                        self.tokenizer.next_token();
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
                while self.tokenizer.current_token != Token::Level(0) {
                    self.tokenizer.next_token();
                }
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

    /// Parses HEAD top-level tag
    fn parse_header(&mut self) -> Header {
        // skip over HEAD tag name
        self.tokenizer.next_token();

        let mut header = Header::default();

        // just skipping the header for now
        while self.tokenizer.current_token != Token::Level(0) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    // TODO: CHAR.VERS
                    "CHAR" => header.encoding = Some(self.take_line_value()),
                    "CORP" => header.corporation = Some(self.take_line_value()),
                    "COPR" => header.copyright = Some(self.take_line_value()),
                    "DATE" => header.date = Some(self.take_line_value()),
                    "DEST" => header.add_destination(self.take_line_value()),
                    "LANG" => header.language = Some(self.take_line_value()),
                    "FILE" => header.filename = Some(self.take_line_value()),
                    "NOTE" => header.note = Some(self.take_continued_text(1)),
                    "SUBM" => header.submitter_tag = Some(self.take_line_value()),
                    "SUBN" => header.submission_tag = Some(self.take_line_value()),
                    "TIME" => {
                        let time = self.take_line_value();
                        // assuming subtag of DATE
                        if let Some(date) = header.date {
                            let mut datetime = String::new();
                            datetime.push_str(&date);
                            datetime.push_str(" ");
                            datetime.push_str(&time);
                            header.date = Some(datetime);
                        } else {
                            panic!("Expected TIME to be under DATE in header.");
                        }
                    }
                    "GEDC" => {
                        header = self.parse_gedcom_data(header);
                    }
                    // TODO: HeaderSource
                    "SOUR" => {
                        println!("WARNING: Skipping header source.");
                        while self.tokenizer.current_token != Token::Level(1) {
                            self.tokenizer.next_token();
                        }
                    }
                    _ => panic!("{} Unhandled Header Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!("Unhandled Header Token: {:?}", self.tokenizer.current_token),
            }
        }
        header
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
                    _ => panic!("{} Unhandled Submitter Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled Submitter Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }
        // println!("found submitter:\n{:#?}", submitter);
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
                        let tag_clone = tag.clone();
                        individual.add_event(self.parse_event(tag_clone.as_str(), level + 1));
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
                    _ => panic!("{} Unhandled Individual Tag: {}", self.dbg(), tag),
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
        // println!("found individual:\n{:#?}", individual);
        individual
    }

    /// Parses FAM top-level tag
    fn parse_family(&mut self, level: u8, xref: Option<String>) -> Family {
        // skip over FAM tag name
        self.tokenizer.next_token();
        let mut family = Family::new(xref);

        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "MARR" => family.add_event(self.parse_event("MARR", level + 1)),
                    "HUSB" => family.set_individual1(self.take_line_value()),
                    "WIFE" => family.set_individual2(self.take_line_value()),
                    "CHIL" => family.add_child(self.take_line_value()),
                    _ => panic!("{} Unhandled Family Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!("Unhandled Family Token: {:?}", self.tokenizer.current_token),
            }
        }

        // println!("found family:\n{:#?}", family);
        family
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
                    "EVEN" => {
                        let events_recorded = self.take_line_value();
                        let mut event = self.parse_event("OTHER", level + 2);
                        event.with_source_data(events_recorded);
                        source.data.add_event(event);
                    }
                    "AGNC" => source.data.agency = Some(self.take_line_value()),
                    "ABBR" => source.abbreviation = Some(self.take_continued_text(level + 1)),
                    "TITL" => source.title = Some(self.take_continued_text(level + 1)),
                    "REPO" => source.add_repo_citation(self.parse_repo_citation(level + 1)),
                    _ => panic!("{} Unhandled Source Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!("Unhandled Source Token: {:?}", self.tokenizer.current_token),
            }
        }

        // println!("found source:\n{:#?}", source);
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
                    _ => panic!("{} Unhandled Repository Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled Repository Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }
        // println!("found repositiory:\n{:#?}", repo);
        repo
    }

    fn parse_custom_tag(&mut self, tag: String) -> CustomData {
        let value = self.take_line_value();
        CustomData { tag, value }
    }

    /// Handle parsing GEDC tag
    fn parse_gedcom_data(&mut self, mut header: Header) -> Header {
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
                _ => panic!(
                    "{} Unexpected GEDC Token: {:?}",
                    self.dbg(),
                    &self.tokenizer.current_token
                ),
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
                    _ => panic!("{} Unhandled FamilyLink Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled FamilyLink Token: {:?}",
                    self.tokenizer.current_token
                ),
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

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
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
                    _ => panic!("{} Unhandled Name Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!("Unhandled Name Token: {:?}", self.tokenizer.current_token),
            }
        }

        name
    }

    fn parse_event(&mut self, tag: &str, level: u8) -> Event {
        self.tokenizer.next_token();
        let mut event = Event::from_tag(tag);
        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATE" => event.date = Some(self.take_line_value()),
                    "PLAC" => event.place = Some(self.take_line_value()),
                    "SOUR" => event.add_citation(self.parse_citation(level + 1)),
                    _ => panic!("{} Unhandled Event Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!("Unhandled Event Token: {:?}", self.tokenizer.current_token),
            }
        }
        event
    }

    /// Parses ADDR tag
    fn parse_address(&mut self, level: u8) -> Address {
        // skip ADDR tag
        self.tokenizer.next_token();
        let mut address = Address::default();
        let mut value = String::new();

        // handle value on ADDR line
        if let Token::LineValue(addr) = &self.tokenizer.current_token {
            value.push_str(addr);
            self.tokenizer.next_token();
        }

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
                        value.push_str(&self.take_line_value());
                    }
                    "ADR1" => address.adr1 = Some(self.take_line_value()),
                    "ADR2" => address.adr2 = Some(self.take_line_value()),
                    "ADR3" => address.adr3 = Some(self.take_line_value()),
                    "CITY" => address.city = Some(self.take_line_value()),
                    "STAE" => address.state = Some(self.take_line_value()),
                    "POST" => address.post = Some(self.take_line_value()),
                    "CTRY" => address.country = Some(self.take_line_value()),
                    _ => panic!("{} Unhandled Address Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled Address Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }

        if &value != "" {
            address.value = Some(value);
        }

        address
    }

    fn parse_citation(&mut self, level: u8) -> SourceCitation {
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
                    _ => panic!("{} Unhandled Citation Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled Citation Token: {:?}",
                    self.tokenizer.current_token
                ),
            }
        }
        citation
    }

    /// Takes the value of the current line including handling
    /// multi-line values from CONT & CONC tags.
    fn take_continued_text(&mut self, level: u8) -> String {
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
    fn take_line_value(&mut self) -> String {
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

    /// Debug function displaying GEDCOM line number of error message.
    fn dbg(&self) -> String {
        format!("line {}:", self.tokenizer.line)
    }
}
