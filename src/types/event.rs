use std::fmt;
use std::string::ToString;

type Xref = String;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub enum EventType {
    Birth,
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
    pub source: Option<Xref>,
}

impl Event {
    pub fn new(etype: EventType) -> Event {
        Event {
            event: etype,
            date: None,
            place: None,
            source: None,
        }
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
