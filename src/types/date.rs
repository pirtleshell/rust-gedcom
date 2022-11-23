use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use super::Note;

/// TODO Date should encompasses a number of date formats, e.g. approximated, period, phrase and range.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Date {
    pub value: Option<String>,
    pub time: Option<String>,
}

impl Date {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Date {
        let mut date = Date::default();
        date.parse(tokenizer, level);
        date
    }

    /// datetime returns Date and Date.time in a single string.
    pub fn datetime(&self) -> Option<String> {
        match &self.time {
            Some(time) => {
                let mut dt = String::new();
                dt.push_str(self.value.as_ref().unwrap().as_str());
                dt.push_str(" ");
                dt.push_str(&time);
                Some(dt)
            }
            None => None,
        }
    }
}

impl Parser for Date {
    /// parse handles the DATE tag
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(tokenizer.take_line_value());

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "TIME" => self.time = Some(tokenizer.take_line_value()),
                    _ => panic!("{} unhandled DATE tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unexpected DATE token: {:?}", tokenizer.current_token),
            }
        }
    }
}

/// ChangeDate is intended to only record the last change to a record. Some systems may want to
/// manage the change process with more detail, but it is sufficient for GEDCOM purposes to
/// indicate the last time that a record was modified.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ChangeDate {
    pub value: Option<String>,
    pub date: Option<Date>,
    pub note: Option<Note>,
}

impl ChangeDate {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> ChangeDate {
        let mut date = ChangeDate::default();
        date.parse(tokenizer, level);
        date
    }
}

impl Parser for ChangeDate {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        tokenizer.next_token();

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
                tokenizer.next_token();
                match &tokenizer.current_token {
                    Token::Tag(tag) => match tag.as_str() {
                        "DATE" => self.date = Some(Date::new(tokenizer, level + 1)),
                        "NOTE" => self.note = Some(Note::new(tokenizer, level + 1)),
                        _ => panic!("{} unhandled ChangeDate tag: {}", tokenizer.debug(), tag),
                    },
                    Token::Level(_) => tokenizer.next_token(),
                    _ => panic!("Unexpected ChangeDate token: {:?}", tokenizer.current_token),
                }
            }
        }
    }
}
