use crate::parser::{Parsable, Parser, ParsingError};
use crate::tokenizer::Token;
use crate::types::SourceCitation;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::{fmt, string::ToString};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum EventType {
    Adoption,
    Birth,
    Burial,
    Death,
    Christening,
    Marriage,
    Residence,
    SourceData(String),

    // "Other" is used to construct an event without requiring an explicit event type
    Other,
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

/// Event fact
#[derive(Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Event {
    pub event: EventType,
    pub date: Option<String>,
    pub place: Option<String>,
    pub citations: Vec<SourceCitation>,
}

impl Event {
    #[must_use]
    pub fn new(etype: EventType) -> Event {
        Event {
            event: etype,
            date: None,
            place: None,
            citations: Vec::new(),
        }
    }

    /** converts an event to be of type `SourceData` with `value` as the data */
    pub fn with_source_data(&mut self, value: String) {
        self.event = EventType::SourceData(value);
    }

    #[must_use]
    pub fn from_tag(tag: &str) -> Event {
        let etype = match tag {
            "ADOP" => EventType::Adoption,
            "BIRT" => EventType::Birth,
            "BURI" => EventType::Burial,
            "CHR" => EventType::Christening,
            "DEAT" => EventType::Death,
            "MARR" => EventType::Marriage,
            "RESI" => EventType::Residence,
            "OTHER" => EventType::Other,
            _ => {
                println!("Unrecognized event tag: {}", tag);
                EventType::Other
            }
        };
        Event::new(etype)
    }

    pub fn add_citation(&mut self, citation: SourceCitation) {
        self.citations.push(citation)
    }

    #[must_use]
    pub fn get_citations(&self) -> Vec<SourceCitation> {
        self.citations.clone()
    }
}

impl Parsable<Event> for Event {
    fn parse(parser: &mut Parser, level: u8) -> Result<Event, ParsingError> {
        // extract current tag name to determine event type.
        let event_tag_token = parser.tokenizer.take_token();
        let tag: &str = if let Token::Tag(t) = &event_tag_token {
            t.as_str().clone()
        } else {
            panic!(
                "Expected event tag, found {:?}",
                &parser.tokenizer.current_token
            );
        };

        let mut event = Event::from_tag(tag);
        loop {
            if let Token::Level(cur_level) = parser.tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &parser.tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATE" => event.date = Some(parser.take_line_value()),
                    "PLAC" => event.place = Some(parser.take_line_value()),
                    // TODO Citation::parse
                    "SOUR" => event.add_citation(parser.parse_citation(level + 1)),
                    _ => panic!("{} Unhandled Event Tag: {}", parser.dbg(), tag),
                },
                Token::Level(_) => {
                    parser.tokenizer.next_token();
                }
                _ => panic!(
                    "Unhandled Event Token: {:?}",
                    parser.tokenizer.current_token
                ),
            }
        }
        Ok(event)
    }
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let event_type = format!("{:?} Event", &self.event);
        let mut debug = f.debug_struct(&event_type);

        fmt_optional_value!(debug, "date", &self.date);
        fmt_optional_value!(debug, "place", &self.place);

        debug.finish()
    }
}

/// Trait given to structs representing entities that have events.
pub trait HasEvents {
    fn add_event(&mut self, event: Event) -> ();
    fn events(&self) -> Vec<Event>;
    fn dates(&self) -> Vec<String> {
        let mut dates: Vec<String> = Vec::new();
        for event in self.events() {
            if let Some(d) = &event.date {
                dates.push(d.clone());
            }
        }
        dates
    }
    fn places(&self) -> Vec<String> {
        let mut places: Vec<String> = Vec::new();
        for event in self.events() {
            if let Some(p) = &event.place {
                places.push(p.clone());
            }
        }
        places
    }
}
