#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
};

/// Translation (tag:TRAN) is a type of TRAN for unstructured human-readable text, such as
/// is found in NOTE and SNOTE payloads. Each NOTE-TRAN must have either a LANG substructure or a
/// MIME substructure or both. If either is missing, it is assumed to have the same value as the
/// superstructure. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#NOTE-TRAN
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Translation {
    pub value: Option<String>,
    /// tag:MIME
    pub mime: Option<String>,
    /// tag:LANG
    pub language: Option<String>,
}

impl Translation {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Translation {
        let mut tran = Translation::default();
        tran.parse(tokenizer, level);
        tran
    }
}

impl Parser for Translation {
    
    ///parse handles the TRAN tag
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
                    "MIME" => self.mime = Some(tokenizer.take_line_value()),
                    "LANG" => self.language = Some(tokenizer.take_line_value()),
                    _ => panic!("{} unhandled NOTE tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unexpected NOTE token: {:?}", &tokenizer.current_token),
            }
        }
    }
}
