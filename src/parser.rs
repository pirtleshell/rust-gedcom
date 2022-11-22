use crate::tokenizer::Tokenizer;

/// Parse converts a subset of a token list into a type's data structure.
pub trait Parser {
    /// parse does the actual parsing of a subset of a token list
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8);
}
