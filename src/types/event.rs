use std::fmt;

type Xref = String;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EventType {
    Birth,
    Death,
    Marriage,
}

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
