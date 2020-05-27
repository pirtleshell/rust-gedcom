use std::{
    panic,
    str::Chars,
};

use crate::tokenizer::{Token, Tokenizer};
use crate::tree::GedcomData;
use crate::types::{
    Address,
    Event,
    Family,
    Gender,
    Individual,
    Name,
    SourceCitation,
    Submitter,
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(chars: Chars<'a>) -> Parser {
        let mut tokenizer = Tokenizer::new(chars);
        tokenizer.next_token();
        Parser { tokenizer }
    }

    pub fn parse_record(&mut self) -> GedcomData {
        let mut data = GedcomData::new();
        loop {
            let level = match self.tokenizer.current_token {
                Token::Level(n) => n,
                _ => panic!("{} Expected Level, found {:?}", self.dbg(), self.tokenizer.current_token),
            };

            self.tokenizer.next_token();

            let mut pointer: Option<String> = None;
            if let Token::Pointer(xref) = &self.tokenizer.current_token {
                pointer = Some(xref.to_string());
                self.tokenizer.next_token();
            }

            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "FAM"  => data.add_family(self.parse_family(level, pointer)),
                    "HEAD" => self.parse_header(),
                    "INDI" => data.add_individual(self.parse_individual(level, pointer)),
                    // "SOUR" => data.add_source(self.parse_source(level, pointer)),
                    "SUBM" => data.add_submitter(self.parse_submitter(level, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("{} Unhandled tag {}", self.dbg(), tag);
                        self.tokenizer.next_token();
                    },
                },
                _ => {
                    println!("{} Unhandled token {:?}", self.dbg(), self.tokenizer.current_token);
                    self.tokenizer.next_token();
                },
            };
        }

        return data;
    }

    fn parse_header(&mut self) {
        // just skipping the header for now
        while self.tokenizer.current_token != Token::Level(0) {
            self.tokenizer.next_token();
        }
        println!("  handled header");
    }

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
                    },
                    "PHON" => submitter.phone = Some(self.take_line_value()),
                    _ => panic!("{} Unhandled Submitter Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Submitter Token: {:?}", self.tokenizer.current_token},
            }
        }
        println!("found submitter:\n{:#?}", submitter);
        return submitter;
    }

    fn parse_individual(&mut self, level: u8, xref: Option<String>) -> Individual {
        // skip over INDI tag name
        self.tokenizer.next_token();
        let mut individual = Individual::empty(xref);

        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => {
                        individual.name = Some(self.parse_name(level + 1));
                    },
                    "SEX" => {
                        individual.sex = self.parse_gender();
                    },
                    "BIRT" | "BURI" | "DEAT" => {
                        let tag_clone = tag.clone();
                        individual.add_event(self.parse_event(tag_clone.as_str(), level + 1));
                    },
                    "FAMC" | "FAMS" => {
                        let tag_copy = tag.clone();
                        match self.parse_string_value(level + 1) {
                            Some(family_xref) => {
                                individual.add_family(family_xref.to_string(), tag_copy.as_str());
                            },
                            None => panic!("No family xref found."),
                        };
                    },
                    _ => panic!("{} Unhandled Individual Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Individual Token: {:?}", self.tokenizer.current_token},
            }
        }
        println!("found individual:\n{:#?}", individual);
        return individual;
    }

    fn parse_family(&mut self, level: u8, xref: Option<String>) -> Family {
        // skip over FAM tag name
        self.tokenizer.next_token();
        let mut family = Family::new(xref);

        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "MARR" => family.add_event(self.parse_event("MARR", level + 1)),
                    "HUSB" => {
                        match self.parse_string_value(level + 1) {
                            Some(xref) => family.set_individual1(xref),
                            None => panic!("No HUSB individual link found."),
                        };
                    },
                    "WIFE" => {
                        match self.parse_string_value(level + 1) {
                            Some(xref) => family.set_individual2(xref),
                            None => panic!("No WIFE individual link found."),
                        };
                    },
                    "CHIL" => {
                        match self.parse_string_value(level + 1) {
                            Some(xref) => family.add_child(xref),
                            None => panic!("No CHIL individual link found."),
                        };
                    },
                    _ => panic!("{} Unhandled Family Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Family Token: {:?}", self.tokenizer.current_token},
            }
        }

        println!("found family:\n{:#?}", family);
        return family;
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
            panic!("Expected gender LineValue, found {:?}", self.tokenizer.current_token);
        }
        self.tokenizer.next_token();
        return gender;
    }

    fn parse_name(&mut self, level: u8) -> Name {
        let mut name = Name {
            value: Some(self.take_line_value()),
            given: None,
            surname: None,
        };

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level { break; }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "GIVN" => name.given = Some(self.take_line_value()),
                    "SURN" => name.surname = Some(self.take_line_value()),
                    _ => panic!("{} Unhandled Name Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Name Token: {:?}", self.tokenizer.current_token},
            }
        }

        return name;
    }

    fn parse_event(&mut self, tag: &str, level: u8) -> Event {
        self.tokenizer.next_token();
        let mut event = Event::from_tag(tag);
        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATE" => event.date = Some(self.take_line_value()),
                    "PLAC" => event.place = Some(self.take_line_value()),
                    "SOUR" => event.add_citation(self.parse_citation(level + 1)),
                    _ => panic!("{} Unhandled Event Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Event Token: {:?}", self.tokenizer.current_token},
            }
        }
        return event;
    }

    fn parse_address(&mut self, level: u8) -> Address {
        // skip ADDR tag
        self.tokenizer.next_token();
        let mut address = Address::new();
        let mut value = String::new();

        // handle value on ADDR line
        if let Token::LineValue(addr) = &self.tokenizer.current_token {
            value.push_str(addr);
            self.tokenizer.next_token();
        }

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level { break; }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CONT" => {
                        value.push('\n');
                        value.push_str(&self.take_line_value());
                    },
                    "ADR1" => { address.adr1 = Some(self.take_line_value()); },
                    "ADR2" => { address.adr2 = Some(self.take_line_value()); },
                    "ADR3" => { address.adr3 = Some(self.take_line_value()); },
                    "CITY" => { address.city = Some(self.take_line_value()); },
                    "STAE" => { address.state = Some(self.take_line_value()); },
                    "POST" => { address.post = Some(self.take_line_value()); },
                    "CTRY" => { address.country = Some(self.take_line_value()); },
                    _ => panic!("{} Unhandled Address Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Address Token: {:?}", self.tokenizer.current_token},
            }
        }

        if &value != "" {
            address.value = Some(value);
        }

        return address;
    }

    fn parse_citation(&mut self, level: u8) -> SourceCitation {
        let mut citation = SourceCitation {
            xref: self.take_line_value(),
            page: None,
        };

        loop {
            if let Token::Level(cur_level) = self.tokenizer.current_token {
                if cur_level <= level { break; }
            }
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PAGE" => citation.page = Some(self.take_line_value()),
                    _ => panic!("{} Unhandled Citation Tag: {}", self.dbg(), tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Citation Token: {:?}", self.tokenizer.current_token},
            }
        }

        println!("found citation:\n{:#?}", citation);
        return citation;
    }

    fn parse_string_value(&mut self, level: u8) -> Option<String> {
        self.tokenizer.next_token();
        let mut ret = String::new();

        if let Token::LineValue(val) = &self.tokenizer.current_token {
            ret.push_str(&val);
        } else {
            return None;
        }

        self.tokenizer.next_token();
        match self.check_and_parse_cont(level) {
            Some(val) => ret.push_str(&val),
            None => (),
        }

        return Some(ret);
    }

    /** checks for CONT tags and returns concatenated string of all consecutive values */
    fn check_and_parse_cont(&mut self, level: u8) -> Option<String> {
        let next_level: u8 = level + 1;

        if self.tokenizer.current_token == Token::Level(next_level) {
            let mut value = String::from(" ");
            self.tokenizer.next_token();

            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CONT" => {
                        value.push_str(&self.take_line_value());

                        // recursively handle more CONT lines
                        match self.check_and_parse_cont(level) {
                            Some(another) => value.push_str(&another),
                            None => (),
                        }
                    },
                    _ => panic!("{} Unexpected tag while parsing for CONT: {}", self.dbg(), tag),
                },
                _ => panic!("Bad accounting in CONT check"),
            };

            return Some(value);
        } else {
            return None;
        }
    }

    fn take_line_value(&mut self) -> String {
        let mut _value = String::new();
        self.tokenizer.next_token();

        if let Token::LineValue(val) = &self.tokenizer.current_token {
            _value = val.to_string();
        } else {
            panic!(
                "{} Expected LineValue, found {:?}",
                self.dbg(),
                self.tokenizer.current_token
            );
        }
        self.tokenizer.next_token();
        return _value;
    }

    fn dbg(&self) -> String {
        format!("line {}:", self.tokenizer.line)
    }
}
