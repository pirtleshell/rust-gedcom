#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    util::{dbg, take_line_value},
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

impl Parse for Copyright {
    /// parse the COPR tag
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
                    "CONT" => self.continued = Some(take_line_value(tokenizer)),
                    "CONC" => self.continued = Some(take_line_value(tokenizer)),
                    _ => panic!("{} unhandled COPR tag in header: {}", dbg(&tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled tag in COPR: {:?}", tokenizer.current_token),
            }
        }
    }
}
