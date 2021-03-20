use crate::parser::{Parsable, Parser, ParsingError};
use crate::tokenizer::Token;
use crate::types::{event::HasEvents, CustomData, Event, FamilyLink};
use std::default::Default;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

/// A Person within the family tree
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Individual {
    pub xref: Option<Xref>,
    pub name: Option<Name>,
    pub sex: Gender,
    pub families: Vec<FamilyLink>,
    pub custom_data: Vec<CustomData>,
    pub last_updated: Option<String>,
    events: Vec<Event>,
}

impl Individual {
    pub fn add_family(&mut self, link: FamilyLink) {
        let mut do_add = true;
        let xref = &link.0;
        for FamilyLink(family, _, _) in &self.families {
            if family.as_str() == xref.as_str() {
                do_add = false;
            }
        }
        if do_add {
            self.families.push(link);
        }
    }

    pub fn add_custom_data(&mut self, data: CustomData) {
        self.custom_data.push(data)
    }
}

impl HasEvents for Individual {
    fn add_event(&mut self, event: Event) -> () {
        self.events.push(event);
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

impl Parsable<Individual> for Individual {
    /// Parses INDI top-level tag
    fn parse(parser: &mut Parser, level: u8) -> Result<Individual, ParsingError> {
        // skip over INDI tag name
        parser.tokenizer.next_token();
        let mut individual = Individual::default();

        while parser.tokenizer.current_token != Token::Level(level) {
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => individual.name = Some(Name::parse(parser, level + 1).unwrap()),
                    "SEX" => individual.sex = Gender::parse(parser, level + 1).unwrap(),
                    "ADOP" | "BIRT" | "BAPM" | "BARM" | "BASM" | "BLES" | "BURI" | "CENS"
                    | "CHR" | "CHRA" | "CONF" | "CREM" | "DEAT" | "EMIG" | "FCOM" | "GRAD"
                    | "IMMI" | "NATU" | "ORDN" | "RETI" | "RESI" | "PROB" | "WILL" | "EVEN" => {
                        individual.add_event(Event::parse(parser, level + 1).unwrap());
                    }
                    "FAMC" | "FAMS" => {
                        individual.add_family(FamilyLink::parse(parser, level + 1).unwrap())
                    }
                    "CHAN" => {
                        // assuming it always only has a single DATE subtag
                        parser.tokenizer.next_token(); // level
                        parser.tokenizer.next_token(); // DATE tag
                        individual.last_updated = Some(parser.take_line_value());
                    }
                    _ => parser.skip_current_tag(level + 1, "Individual"),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    individual.add_custom_data(parser.parse_custom_tag(tag_clone))
                }
                Token::Level(_) => parser.tokenizer.next_token(),
                _ => panic!(
                    "Unhandled Individual Token: {:?}",
                    parser.tokenizer.current_token
                ),
            }
        }

        Ok(individual)
    }
}

/// Gender of an `Individual`
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum Gender {
    Male,
    Female,
    // come at me LDS, i support "N" as a gender value
    Nonbinary,
    Unknown,
}

impl Default for Gender {
    fn default() -> Gender {
        Gender::Unknown
    }
}

impl Parsable<Gender> for Gender {
    fn parse(parser: &mut Parser, _level: u8) -> Result<Gender, ParsingError> {
        parser.tokenizer.next_token();
        let gender: Gender;
        if let Token::LineValue(gender_string) = &parser.tokenizer.current_token {
            gender = match gender_string.as_str() {
                "M" => Gender::Male,
                "F" => Gender::Female,
                "N" => Gender::Nonbinary,
                "U" => Gender::Unknown,
                _ => panic!("{} Unknown gender value {}", parser.dbg(), gender_string),
            };
        } else {
            panic!(
                "Expected gender LineValue, found {:?}",
                parser.tokenizer.current_token
            );
        }
        parser.tokenizer.next_token();

        Ok(gender)
    }
}

#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Name {
    pub value: Option<String>,
    pub given: Option<String>,
    pub surname: Option<String>,
    pub prefix: Option<String>,
    pub surname_prefix: Option<String>,
    pub suffix: Option<String>,
}

impl Parsable<Name> for Name {
    fn parse(parser: &mut Parser, level: u8) -> Result<Name, ParsingError> {
        let mut name = Name::default();
        name.value = Some(parser.take_line_value());
        let mut cur_level = level;

        loop {
            if let Token::Level(new_level) = parser.tokenizer.current_token {
                if new_level <= cur_level {
                    break;
                }
            }
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "GIVN" => name.given = Some(parser.take_line_value()),
                    "NPFX" => name.prefix = Some(parser.take_line_value()),
                    "NSFX" => name.suffix = Some(parser.take_line_value()),
                    "SPFX" => name.surname_prefix = Some(parser.take_line_value()),
                    "SURN" => name.surname = Some(parser.take_line_value()),
                    _ => parser.skip_current_tag(cur_level, "Name"),
                },
                Token::Level(_) => {
                    cur_level += 1;
                    parser.tokenizer.next_token()
                }
                _ => panic!("Unhandled Name Token: {:?}", parser.tokenizer.current_token),
            }
        }

        Ok(name)
    }
}
