use crate::types::SourceCitation;
use std::{fmt, string::ToString};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, PartialEq)]
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
            "CHR"  => EventType::Christening,
            "DEAT" => EventType::Death,
            "MARR" => EventType::Marriage,
            "RESI" => EventType::Residence,
            "OTHER" => EventType::Other,
            _ => panic!("Unrecognized event tag: {}", tag),
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

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let event_type = format!("{:?} Event", &self.event);
        let mut debug = f.debug_struct(&event_type);

        fmt_optional_value!(debug, "date", &self.date);
        fmt_optional_value!(debug, "place", &self.place);

        debug.finish()
    }
}
