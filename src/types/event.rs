use std::fmt;

#[derive(Debug)]
pub enum EventType {
    Birth,
    Death,
    Marriage,
}

pub struct Event {
    pub event: EventType,
    pub date: Option<String>,
    pub place: Option<String>,
}

impl Event {
    pub fn new(etype: EventType) -> Event {
        Event {
            event: etype,
            date: None,
            place: None,
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
