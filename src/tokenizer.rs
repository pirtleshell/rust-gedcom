use std::str::Chars;

// making use of FamilySearch's GEDCOM Standard Release 5.5.1
// https://www.familysearch.org/wiki/en/GEDCOM
// gedcom_line: level + delim + [optional_xref_ID] + tag + [optional_line_value] + terminator
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Level(u8),
    Tag(String),
    LineValue(String),
    Pointer(String),
    EOF,
    None,
}

pub struct Tokenizer<'a> {
    pub current_token: Token,
    current_char: char,
    chars: Chars<'a>,
    pub line: u32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(mut chars: Chars<'a>) -> Tokenizer {
        Tokenizer {
            current_char: '\n',
            current_token: Token::None,
            chars,
            line: 0,
        }
    }

    pub fn done(&self) -> bool {
        self.current_token == Token::EOF
    }

    pub fn next_token(&mut self) {
        if self.current_char == '\0' {
            self.current_token = Token::EOF;
            return;
        }

        // level number is at the start of each line.
        if self.current_char == '\r' { self.next_char(); }
        if self.current_char == '\n' {
            self.next_char();
            self.current_token = Token::Level(self.extract_number());
            self.line += 1;
            return;
        }

        self.skip_whitespace();

        self.current_token = match self.current_token {
            Token::Level(_) => {
                if self.current_char == '@' {
                    Token::Pointer(self.extract_word())
                } else {
                    Token::Tag(self.extract_word())
                }
            },
            Token::Pointer(_) => { Token::Tag(self.extract_word() )},
            Token::Tag(_) => { Token::LineValue(self.extract_value()) },
            _ => panic!("Tokenization error!"),
        };
    }

    fn next_char(&mut self) {
        self.current_char = self.chars.next().unwrap_or('\0');
    }

    fn extract_number(&mut self) -> u8 {
        let mut digits: Vec<char> = Vec::new();
        while self.current_char.is_digit(10) {
            digits.push(self.current_char);
            self.next_char();
        }

        digits.iter().collect::<String>().parse::<u8>().unwrap()
    }

    fn extract_word(&mut self) -> String {
        let mut letters: Vec<char> = Vec::new();
        while !self.current_char.is_whitespace() && self.current_char != '\0' {
            letters.push(self.current_char);
            self.next_char();
        }

        letters.iter().collect::<String>()
    }

    fn extract_value(&mut self) -> String {
        let mut letters: Vec<char> = Vec::new();
        while self.current_char != '\n' {
            letters.push(self.current_char);
            self.next_char();
        }

        letters.iter().collect::<String>()
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.next_char();
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn parses_tokens() {}
// }
