use crate::{
    tokenizer::{Token, Tokenizer},
    types::{Date, FamilyLink, Note, SourceCitation},
    Parser,
};

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
    Cremation,
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
    pub value: Option<String>,
    pub date: Option<Date>,
    pub place: Option<String>,
    pub note: Option<Note>,
    pub child_to_family_link: Option<FamilyLink>,
    pub citations: Vec<SourceCitation>,
}

impl Event {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, tag: &str) -> Event {
        let mut event = Event {
            event: Self::from_tag(tag),
            value: None,
            date: None,
            place: None,
            note: None,
            child_to_family_link: None,
            citations: Vec::new(),
        };
        event.parse(tokenizer, level);
        event
    }

    /** converts an event to be of type `SourceData` with `value` as the data */
    pub fn with_source_data(&mut self, value: String) {
        self.event = EventType::SourceData(value);
    }

    pub fn from_tag(tag: &str) -> EventType {
        match tag {
            "ADOP" => EventType::Adoption,
            "BIRT" => EventType::Birth,
            "BURI" => EventType::Burial,
            "CHR" => EventType::Christening,
            "DEAT" => EventType::Death,
            "MARR" => EventType::Marriage,
            "CREM" => EventType::Cremation,
            "RESI" => EventType::Residence,
            "OTHER" => EventType::Other,
            _ => panic!("Unrecognized EventType tag: {}", tag),
        }
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

/// Trait given to structs representing entities that have events.
pub trait HasEvents {
    fn add_event(&mut self, event: Event) -> ();
    fn events(&self) -> Vec<Event>;
    fn dates(&self) -> Vec<Date> {
        let mut dates: Vec<Date> = Vec::new();
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

impl Parser for Event {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        tokenizer.next_token();

        // handle value on event line
        let mut value = String::new();

        if let Token::LineValue(val) = &tokenizer.current_token {
            value.push_str(&val);
            tokenizer.next_token();
        }

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }

            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "DATE" => self.date = Some(Date::new(tokenizer, level + 1)),
                    "PLAC" => self.place = Some(tokenizer.take_line_value()),
                    "SOUR" => self.add_citation(SourceCitation::new(tokenizer, level + 1)),
                    "FAMC" => {
                        let tag_clone = tag.clone();
                        self.child_to_family_link =
                            Some(FamilyLink::new(tokenizer, level + 1, tag_clone.as_str()))
                    }
                    "NOTE" => self.note = Some(Note::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled Event Tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Event Token: {:?}", tokenizer.current_token),
            }
        }

        if &value != "" {
            self.value = Some(value);
        }
    }
}
