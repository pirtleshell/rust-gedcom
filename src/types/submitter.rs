use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    types::Address,
    util::{dbg, take_line_value},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

/// Submitter of the data, ie. who reported the genealogy fact
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Submitter {
    /// Optional reference to link to this submitter
    pub xref: Option<Xref>,
    /// Name of the submitter
    pub name: Option<String>,
    /// Physical address of the submitter
    pub address: Option<Address>,
    /// Phone number of the submitter
    pub phone: Option<String>,
    /// TODO
    pub language: Option<String>,
}

impl Submitter {
    /// Shorthand for creating a `Submitter` from its `xref`
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> Submitter {
        let mut subm = Submitter::default();
        subm.xref = xref;
        subm.parse(tokenizer, level);
        subm
    }
}

impl Parse for Submitter {
    /// Parse handles SUBM top-level tag
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {

        // skip over SUBM tag name
        tokenizer.next_token();

        while tokenizer.current_token != Token::Level(level) {
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => self.name = Some(take_line_value(tokenizer)),
                    "ADDR" => self.address = Some(Address::new(tokenizer, level + 1)),
                    "PHON" => self.phone = Some(take_line_value(tokenizer)),
                    "LANG" => self.language = Some(take_line_value(tokenizer)),
                    // TODO
                    // "CHAN" => submitter.change_date = Some(take_line_value(&mut self.tokenizer)),
                    _ => panic!("{} Unhandled Submitter Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Submitter Token: {:?}", tokenizer.current_token),
            }
        }
    }
}
