use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
    types::{Event, RepoCitation, UserDefinedData},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use super::{Xref};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// Source for genealogy facts
pub struct Source {
    pub xref: Option<String>,
    pub data: SourceData,
    pub abbreviation: Option<String>,
    pub title: Option<String>,
    repo_citations: Vec<RepoCitation>,
}

impl Source {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<String>) -> Source {
        let mut sour = Source {
            xref,
            data: SourceData {
                events: Vec::new(),
                agency: None,
            },
            abbreviation: None,
            title: None,
            repo_citations: Vec::new(),
        };
        sour.parse(tokenizer, level);
        sour
    }

    pub fn add_repo_citation(&mut self, citation: RepoCitation) {
        self.repo_citations.push(citation);
    }
}

impl Parser for Source {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip SOUR tag
        tokenizer.next_token();

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATA" => tokenizer.next_token(),
                    "EVEN" => {
                        let events_recorded = tokenizer.take_line_value();
                        let mut event = Event::new(tokenizer, level + 2, "OTHER");
                        event.with_source_data(events_recorded);
                        self.data.add_event(event);
                    }
                    "AGNC" => self.data.agency = Some(tokenizer.take_line_value()),
                    "ABBR" => self.abbreviation = Some(tokenizer.take_continued_text(level + 1)),
                    "TITL" => self.title = Some(tokenizer.take_continued_text(level + 1)),
                    "REPO" => self.add_repo_citation(RepoCitation::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled Source Tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Source Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct SourceData {
    events: Vec<Event>,
    pub agency: Option<String>,
}

impl SourceData {
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

/// Citation linking a genealogy fact to a data `Source`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct SourceCitation {
    /// Reference to the `Source`
    pub xref: Xref,
    /// Page number of source
    pub page: Option<String>,
    pub custom_data: Vec<UserDefinedData>,
}

impl SourceCitation {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> SourceCitation {
        let mut citation = SourceCitation {
            xref: tokenizer.take_line_value(),
            page: None,
            custom_data: Vec::new(),
        };
        citation.parse(tokenizer, level);
        citation
    }

    pub fn add_custom_data(&mut self, data: UserDefinedData) {
        self.custom_data.push(data)
    }
}

impl Parser for SourceCitation {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PAGE" => self.page = Some(tokenizer.take_line_value()),
                    _ => panic!("{} Unhandled Citation Tag: {}", tokenizer.debug(), tag),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    self.add_custom_data(tokenizer.parse_custom_tag(tag_clone))
                }
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Citation Token: {:?}", tokenizer.current_token),
            }
        }
    }
}
