use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    util::{dbg, take_line_value},
};

use super::{Address, Xref};

/// Data repository, the `REPO` tag
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Repository {
    /// Optional reference to link to this repo
    pub xref: Option<Xref>,
    /// Name of the repository
    pub name: Option<String>,
    /// Physical address of the data repository
    pub address: Option<Address>,
}

impl Repository {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<String>) -> Repository {
        let mut repo = Repository {
            xref,
            name: None,
            address: None,
        };
        repo.parse(tokenizer, level);
        repo
    }
}

impl Parse for Repository {
    /// Parses REPO top-level tag.
    fn parse(&mut self, tokenizer: &mut crate::tokenizer::Tokenizer, level: u8) {
        // skip REPO tag
        tokenizer.next_token();

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => self.name = Some(take_line_value(tokenizer)),
                    "ADDR" => self.address = Some(Address::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled Repository Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Repository Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

/// Citation linking a `Source` to a data `Repository`
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct RepoCitation {
    /// Reference to the `Repository`
    pub xref: Xref,
    /// Call number to find the source at this repository
    pub call_number: Option<String>,
}

impl RepoCitation {
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> RepoCitation {
        let mut rc = RepoCitation {
            xref: take_line_value(tokenizer),
            call_number: None,
        };
        rc.parse(tokenizer, level);
        rc
    }
}

impl Parse for RepoCitation {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "CALN" => self.call_number = Some(take_line_value(tokenizer)),
                    _ => panic!("{} Unhandled RepoCitation Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "Unhandled RepoCitation Token: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}
