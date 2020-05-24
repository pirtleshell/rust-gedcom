use std::str::Chars;

use crate::tokenizer::{Token, Tokenizer};
use crate::tree::GedcomData;
use crate::types::{
    Event,
    EventType,
    Gender,
    Individual,
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
                _ => panic!("Expected Level, found {:?}", self.tokenizer.current_token),
            };

            println!("Level {} record", level);
            self.tokenizer.next_token();

            let mut pointer: Option<String> = None;
            if let Token::Pointer(xref) = &self.tokenizer.current_token {
                println!("  found pointer: {}", xref);
                pointer = Some(xref.to_string());
                self.tokenizer.next_token();
            }

            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "HEAD" => self.parse_header(),
                    "SUBM" => data.add_submitter(self.parse_submitter(level, pointer)),
                    "INDI" => data.add_individual(self.parse_individual(level, pointer)),
                    "TRLR" => break,
                    _ => {
                        println!("Unhandled tag {}", tag);
                        self.tokenizer.next_token();
                    },
                },
                _ => {
                    println!("! Unhandled token {:?}", self.tokenizer.current_token);
                    self.tokenizer.next_token();
                },
            };

            if let Token::LineValue(val) = &self.tokenizer.current_token {
                println!("  has value {}", val);
                self.tokenizer.next_token();
            }
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

        let mut submitter = Submitter {
            name: None,
            address: None,
            xref,
        };
        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => {
                        submitter.name = self.parse_string_value(level + 1);
                    },
                    "ADDR" => {
                        submitter.address = self.parse_string_value(level + 1);
                    },
                    _ => panic!("Unhandled Submitter Tag: {}", tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Submitter Token: {:?}", self.tokenizer.current_token},
            }
        }
        println!("found submitter:\n{:?}", submitter);
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
                        individual.name = self.parse_string_value(level + 1);
                    },
                    "SEX" => {
                        individual.sex = self.parse_gender();
                    },
                    "BIRT" => {
                        individual.birth = Some(self.parse_event(EventType::Birth, level + 1));
                    },
                    "DEAT" => {
                        individual.death = Some(self.parse_event(EventType::Death, level + 1));
                    },
                    "FAMC" | "FAMS" => {
                        let tag_copy = tag.clone();
                        match self.parse_string_value(level + 1) {
                            Some(family_xref) => {
                                individual.add_family(family_xref.to_string(), tag_copy.as_str());
                            },
                            None => panic!("No family xref found."),
                        };
                    }
                    _ => panic!("Unhandled Individual Tag: {}", tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Individual Token: {:?}", self.tokenizer.current_token},
            }
        }
        println!("found individual:\n{:#?}", individual);
        return individual;
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
                _ => panic!("Unknown gender value {}", gender_string),
            };
        } else {
            panic!("Expected gender LineValue, found {:?}", self.tokenizer.current_token);
        }
        self.tokenizer.next_token();
        return gender;
    }

    fn parse_event(&mut self, etype: EventType, level: u8) -> Event {
        self.tokenizer.next_token();
        let mut event = Event::new(etype);
        while self.tokenizer.current_token != Token::Level(level) {
            match &self.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PLAC" => { event.place = self.parse_string_value(level + 1); },
                    "DATE" => { event.date = self.parse_string_value(level + 1); },
                    _ => panic!("Unhandled Event Tag: {}", tag),
                },
                Token::Level(_) => self.tokenizer.next_token(),
                _ => panic!{"Unhandled Event Token: {:?}", self.tokenizer.current_token},
            }
        }
        return event;
    }

    fn parse_string_value(&mut self, level: u8) -> Option<String> {
        self.tokenizer.next_token();
        let mut ret = String::new();

        if let Token::LineValue(val) = &self.tokenizer.current_token {
            ret.push_str(&val);
        } else {
            return None;
        }

        println!("found value {}", ret);

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
                        self.tokenizer.next_token();
                        if let Token::LineValue(val) = &self.tokenizer.current_token {
                            value.push_str(&val);
                            self.tokenizer.next_token();

                            // recursively handle more CONT lines
                            match self.check_and_parse_cont(level) {
                                Some(another) => value.push_str(&another),
                                None => (),
                            }
                        } else {
                            panic!("Expected LineValue, found {:?}", self.tokenizer.current_token);
                        }
                    },
                    _ => panic!("Unexpected tag while parsing for CONT: {}", tag),
                },
                _ => panic!("Bad accounting in CONT check"),
            };

            return Some(value);
        } else {
            return None;
        }
    }

}
