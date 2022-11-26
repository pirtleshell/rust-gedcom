use crate::{
    Parser,
    tokenizer::{Token, Tokenizer},
    types::Address,
};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Corporation (tag: CORP) is the name of the business, corporation, or person that produced or
/// commissioned the product. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#CORP
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Corporation {
    pub value: Option<String>,
    /// tag: ADDR
    pub address: Option<Address>,
    /// tag: PHON
    pub phone: Option<String>,
    /// tag: EMAIL
    pub email: Option<String>,
    /// tag: FAX
    pub fax: Option<String>,
    /// tag: WWW
    pub website: Option<String>,
}


impl Corporation {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Corporation {
        let mut corp = Corporation::default();
        corp.parse(tokenizer, level);
        corp
    }
}

impl Parser for Corporation {
    /// parse is for a CORP tag within the SOUR tag of a HEADER
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
                    "ADDR" => self.address = Some(Address::new(tokenizer, level + 1)),
                    "PHON" => self.phone = Some(tokenizer.take_line_value()),
                    "EMAIL" => self.email = Some(tokenizer.take_line_value()),
                    "FAX" => self.fax = Some(tokenizer.take_line_value()),
                    "WWW" => self.website = Some(tokenizer.take_line_value()),
                    _ => panic!("{} Unhandled CORP tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "Unhandled CORP tag in header: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}
