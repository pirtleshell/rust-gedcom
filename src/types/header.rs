use crate::parser::{Parsable, Parser, ParsingError};
use crate::tokenizer::Token;
use crate::types::Source;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// Header containing GEDCOM metadata
pub struct Header {
    pub encoding: Option<String>,
    pub copyright: Option<String>,
    pub corporation: Option<String>,
    pub date: Option<String>,
    pub destinations: Vec<String>,
    pub gedcom_version: Option<String>,
    pub language: Option<String>,
    pub filename: Option<String>,
    pub note: Option<String>,
    pub sources: Vec<Source>,
    pub submitter_tag: Option<String>,
    pub submission_tag: Option<String>,
}

impl Header {
    pub fn add_destination(&mut self, destination: String) {
        self.destinations.push(destination);
    }

    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }
}

// pub struct HeaderSource {
//     version: Option<String>,
//     name: Option<String>,
//     coroporation:
// }

impl Parsable<Header> for Header {
    /// Parses HEAD top-level tag
    fn parse(parser: &mut Parser) -> Result<Header, ParsingError> {
        let base_lvl = parser.level;
        // skip over HEAD tag name
        parser.tokenizer.next_token();

        let mut header = Header::default();

        // just skipping the header for now
        while parser.tokenizer.current_token != Token::Level(base_lvl) {
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    // TODO: CHAR.VERS - version
                    "CHAR" => header.encoding = Some(parser.take_line_value()),
                    "CORP" => header.corporation = Some(parser.take_line_value()),
                    "COPR" => header.copyright = Some(parser.take_line_value()),
                    "DATE" => header.date = Some(parser.take_line_value()),
                    "DEST" => header.add_destination(parser.take_line_value()),
                    "LANG" => header.language = Some(parser.take_line_value()),
                    "FILE" => header.filename = Some(parser.take_line_value()),
                    "NOTE" => header.note = Some(parser.take_continued_text(1)),
                    "SUBM" => header.submitter_tag = Some(parser.take_line_value()),
                    "SUBN" => header.submission_tag = Some(parser.take_line_value()),
                    "TIME" => {
                        let time = parser.take_line_value();
                        // assuming subtag of DATE
                        if let Some(date) = header.date {
                            let mut datetime = String::new();
                            datetime.push_str(&date);
                            datetime.push_str(" ");
                            datetime.push_str(&time);
                            header.date = Some(datetime);
                        } else {
                            panic!("Expected TIME to be under DATE in header.");
                        }
                    }
                    "GEDC" => {
                        header = parser.parse_gedcom_data(header);
                    }
                    // TODO: HeaderSource
                    "SOUR" => {
                        println!("WARNING: Skipping header source.");
                        while parser.tokenizer.current_token != Token::Level(1) {
                            parser.tokenizer.next_token();
                        }
                    }
                    _ => parser.skip_current_tag("Header"),
                },
                Token::Level(_) => parser.set_level(),
                _ => parser.handle_unexpected_token("HEAD"),
            }
        }

        Ok(header)
    }
}
