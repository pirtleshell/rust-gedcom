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
    Baptism,
    BarMitzvah,
    BasMitzvah,
    Birth,
    Blessing,
    Burial,
    Census,
    Christening,
    ChristeningAdult,
    Confirmation,
    Cremation,
    Death,
    Emigration,
    FirstCommunion,
    Graduation,
    Immigration,
    Marriage,
    Naturalization,
    Ordination,
    Probate,
    Residence,
    Retirement,
    Will,
    SourceData(String),

    // "Other" is used to construct an event without requiring an explicit event type
    Other,
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl EventType {
    #[must_use]
    pub fn from_tag(tag: &str) -> EventType {
        match tag {
            "ADOP" => EventType::Adoption,
            "BAPM" => EventType::Baptism,
            "BARM" => EventType::BarMitzvah,
            "BASM" => EventType::BasMitzvah,
            "BLES" => EventType::Blessing,
            "BIRT" => EventType::Birth,
            "BURI" => EventType::Burial,
            "CENS" => EventType::Census,
            "CHR" => EventType::Christening,
            "CHRA" => EventType::ChristeningAdult,
            "CONF" => EventType::Confirmation,
            "CREM" => EventType::Cremation,
            "DEAT" => EventType::Death,
            "EMIG" => EventType::Emigration,
            "FCOM" => EventType::FirstCommunion,
            "GRAD" => EventType::Graduation,
            "IMMI" => EventType::Immigration,
            "MARR" => EventType::Marriage,
            "NATU" => EventType::Naturalization,
            "ORDN" => EventType::Ordination,
            "PROB" => EventType::Probate,
            "RESI" => EventType::Residence,
            "RETI" => EventType::Retirement,
            "WILL" => EventType::Will,

            "OTHER" => EventType::Other,

            "EVEN" => panic!("EVEN passed as event tag instead of value."),
            _ => panic!("Unrecognized event tag: {}", tag),
        }
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
        let etype = EventType::from_tag(tag);
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

        // Events begin with either EVEN <type>, or a type tag.
        let type_tag: &str = if tag == "EVEN" {
            if let Token::LineValue(v) = &parser.tokenizer.current_token {
                v
            } else {
                // if there's no line value, there's supposed to be a TYPE tag
                "OTHER"
            }
        } else {
            tag
        };

        let mut event = Event::from_tag(&type_tag);

        parser.tokenizer.next_token();

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
                    _ => parser.skip_current_tag(level + 1, "Event"),
                },
                Token::Level(_) => parser.tokenizer.next_token(),
                // some events are also bool-like w/ Y values, apparently?
                Token::LineValue(v) => {
                    // TODO: return error and stop using dbg
                    if v.as_str() != "Y" {
                        panic!("{} Surprise value {} as event value", parser.dbg(), v);
                    }
                    // just skip Y's
                    parser.tokenizer.next_token();
                }
                _ => parser.handle_unexpected_token(level + 1, "Event"),
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
