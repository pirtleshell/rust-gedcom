#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
    util::{dbg, take_line_value},
};

/// Physical address at which a fact occurs
#[derive(Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Address {
    pub value: Option<String>,
    pub adr1: Option<String>,
    pub adr2: Option<String>,
    pub adr3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub post: Option<String>,
    pub country: Option<String>,
}

impl Address {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Address {
        let mut addr = Address::default();
        addr.parse(tokenizer, level);
        addr
    }
}

impl Parser for Address {
    /// parse handles ADDR tag
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip ADDR tag
        tokenizer.next_token();

        let mut value = String::new();

        // handle value on ADDR line
        if let Token::LineValue(addr) = &tokenizer.current_token {
            value.push_str(&addr);
            tokenizer.next_token();
        }

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CONT" | "CONC" => {
                        value.push('\n');
                        value.push_str(&take_line_value(tokenizer));
                    }
                    "ADR1" => self.adr1 = Some(take_line_value(tokenizer)),
                    "ADR2" => self.adr2 = Some(take_line_value(tokenizer)),
                    "ADR3" => self.adr3 = Some(take_line_value(tokenizer)),
                    "CITY" => self.city = Some(take_line_value(tokenizer)),
                    "STAE" => self.state = Some(take_line_value(tokenizer)),
                    "POST" => self.post = Some(take_line_value(tokenizer)),
                    "CTRY" => self.country = Some(take_line_value(tokenizer)),
                    _ => panic!("{} Unhandled Address Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Address Token: {:?}", tokenizer.current_token),
            }
        }

        if &value != "" {
            self.value = Some(value);
        }
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Address");

        fmt_optional_value!(debug, "value", &self.value);
        fmt_optional_value!(debug, "adr1", &self.adr1);
        fmt_optional_value!(debug, "adr2", &self.adr2);
        fmt_optional_value!(debug, "adr3", &self.adr3);
        fmt_optional_value!(debug, "city", &self.city);
        fmt_optional_value!(debug, "state", &self.state);
        fmt_optional_value!(debug, "post", &self.post);
        fmt_optional_value!(debug, "country", &self.country);

        debug.finish()
    }
}
