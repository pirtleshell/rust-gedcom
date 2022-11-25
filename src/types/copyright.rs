#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
};

/// A copyright statement, as appropriate for the copyright laws applicable to this data.
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#COPR
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Copyright {
    pub value: Option<String>,
    /// tag: CONT
    pub continued: Option<String>,
}

impl Copyright {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Copyright {
        let mut copr = Copyright::default();
        copr.parse(tokenizer, level);
        copr
    }
}

impl Parser for Copyright {
    /// parse the COPR tag
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
                    "CONT" => self.continued = Some(tokenizer.take_line_value()),
                    "CONC" => self.continued = Some(tokenizer.take_line_value()),
                    _ => panic!(
                        "{} unhandled COPR tag in header: {}",
                        tokenizer.debug(),
                        tag
                    ),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled tag in COPR: {:?}", tokenizer.current_token),
            }
        }
    }
}
