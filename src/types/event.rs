use std::{
    fmt,
    string::ToString,
};
use crate::types::SourceCitation;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub enum EventType {
    Birth,
    Burial,
    Death,
    Marriage,
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Clone)]
pub struct Event {
    pub event: EventType,
    pub date: Option<String>,
    pub place: Option<String>,
    pub citations: Vec<SourceCitation>,
}

impl Event {
    pub fn new(etype: EventType) -> Event {
        Event {
            event: etype,
            date: None,
            place: None,
            citations: Vec::new(),
        }
    }

    pub fn from_tag(tag: &str) -> Event {
        let etype = match tag {
            "BIRT" => EventType::Birth,
            "BURI" => EventType::Burial,
            "DEAT" => EventType::Death,
            "MARR" => EventType::Marriage,
            _ => panic!("Unrecognized event tag: {}", tag),
        };
        Event::new(etype)
    }

    pub fn add_citation(&mut self, citation: SourceCitation) {
        self.citations.push(citation)
    }

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
