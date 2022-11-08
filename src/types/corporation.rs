use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    types::Address,
    util::{dbg, take_line_value},
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

impl Parse for Corporation {
    /// parse is for a CORP tag within the SOUR tag of a HEADER
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
                    "ADDR" => self.address = Some(Address::new(tokenizer, level + 1)),
                    "PHON" => self.phone = Some(take_line_value(tokenizer)),
                    "EMAIL" => self.email = Some(take_line_value(tokenizer)),
                    "FAX" => self.fax = Some(take_line_value(tokenizer)),
                    "WWW" => self.website = Some(take_line_value(tokenizer)),
                    _ => panic!("{} Unhandled CORP tag: {}", dbg(tokenizer), tag),
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
