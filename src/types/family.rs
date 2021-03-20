use crate::parser::{Parsable, Parser, ParsingError};
use crate::tokenizer::Token;
use crate::types::{event::HasEvents, Event};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

/// Family fact, representing a relationship between `Individual`s
///
/// This data representation understands that HUSB & WIFE are just poorly-named
/// pointers to individuals. no gender "validating" is done on parse.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Family {
    pub xref: Option<Xref>,
    /// mapped from HUSB
    pub individual1: Option<Xref>,
    /// mapped from WIFE
    pub individual2: Option<Xref>,
    pub children: Vec<Xref>,
    pub num_children: Option<u8>,
    events: Vec<Event>,
}

impl Family {
    pub fn add_child(&mut self, xref: Xref) {
        self.children.push(xref);
    }

    pub fn set_individual1(&mut self, xref: Xref) {
        match self.individual1 {
            Some(_) => panic!("First individual of family already exists."),
            None => self.individual1 = Some(xref),
        };
    }

    pub fn set_individual2(&mut self, xref: Xref) {
        match self.individual2 {
            Some(_) => panic!("Second individual of family already exists."),
            None => self.individual2 = Some(xref),
        };
    }

    #[must_use]
    pub fn with_xref(mut self, xref: Option<Xref>) -> Family {
        self.xref = xref;
        self
    }
}

impl HasEvents for Family {
    fn add_event(&mut self, event: Event) -> () {
        self.events.push(event);
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

impl Parsable<Family> for Family {
    /// Parses FAM top-level tag
    fn parse(parser: &mut Parser) -> Result<Family, ParsingError> {
        let base_lvl = parser.level;
        // skip over FAM tag name
        parser.tokenizer.next_token();
        let mut family = Family::default();

        while parser.tokenizer.current_token != Token::Level(base_lvl) {
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "MARR" => family.add_event(Event::parse(parser).unwrap()),
                    "HUSB" => family.set_individual1(parser.take_line_value()),
                    "WIFE" => family.set_individual2(parser.take_line_value()),
                    "CHIL" => family.add_child(parser.take_line_value()),
                    _ => parser.skip_current_tag("Family"),
                },
                Token::Level(_) => parser.set_level(),
                _ => parser.handle_unexpected_token("FAM"),
            }
        }

        Ok(family)
    }
}
