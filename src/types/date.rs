use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    util::{dbg, take_line_value},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// TODO Date should encompasses a number of date formats, e.g. approximated, period, phrase and range.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Date {
    pub value: Option<String>,
    pub time: Option<String>,
}

impl Date {
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

impl Date {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Date {
        let mut date = Date::default();
        date.parse(tokenizer, level);
        date
    }
}

impl Parse for Date {
    /// parse handles the DATE tag
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(take_line_value(tokenizer));

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "TIME" => self.time = Some(take_line_value(tokenizer)),
                    _ => panic!("{} unhandled DATE tag: {}", dbg(tokenizer), tag),
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
    pub date: Option<Date>,
    pub note: Option<String>,
}
