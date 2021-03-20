use crate::parser::{Parsable, Parser, ParsingError};
use crate::tokenizer::Token;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Physical address at which a fact occurs
#[derive(Default, Debug)]
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

impl fmt::Display for Address {
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

impl Parsable<Address> for Address {
    fn parse(parser: &mut Parser) -> Result<Address, ParsingError> {
        let base_lvl = parser.level;
        // skip ADDR tag
        if let Token::Tag(_) = &parser.tokenizer.current_token {
            parser.tokenizer.next_token();
        }

        let mut address = Address::default();
        let mut value = String::new();

        // handle value on ADDR line
        if let Token::LineValue(addr) = &parser.tokenizer.current_token {
            value.push_str(addr);
            parser.tokenizer.next_token();
        }

        loop {
            if let Token::Level(cur_level) = parser.tokenizer.current_token {
                if cur_level <= base_lvl {
                    break;
                }
            }
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CONT" => {
                        value.push('\n');
                        value.push_str(&parser.take_line_value());
                    }
                    "ADR1" => address.adr1 = Some(parser.take_line_value()),
                    "ADR2" => address.adr2 = Some(parser.take_line_value()),
                    "ADR3" => address.adr3 = Some(parser.take_line_value()),
                    "CITY" => address.city = Some(parser.take_line_value()),
                    "STAE" => address.state = Some(parser.take_line_value()),
                    "POST" => address.post = Some(parser.take_line_value()),
                    "CTRY" => address.country = Some(parser.take_line_value()),
                    // TODO ParsingError
                    _ => parser.skip_current_tag(parser.level, "Address"),
                },
                Token::Level(_) => parser.set_level(),
                _ => panic!(
                    "Unhandled Address Token: {:?}",
                    parser.tokenizer.current_token
                ),
            }
        }

        if &value != "" {
            address.value = Some(value);
        }
        Ok(address)
    }
}
