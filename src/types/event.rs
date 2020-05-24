use std::fmt;

#[derive(Debug)]
pub enum EventType {
    Birth,
    Death,
    Marriage,
}

pub struct Event {
    pub event: EventType,
    pub place: Option<String>,
    pub date: Option<String>,
}

impl Event {
    pub fn new(etype: EventType) -> Event {
        Event {
            event: etype,
            place: None,
            date: None,
        }
    }
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let event_type = format!("{:?} Event", &self.event);
        let mut debug = f.debug_struct(&event_type);

        if let Some(date) = &self.date {
            debug.field("date", date);
        }

        if let Some(place) = &self.place {
            debug.field("place", place);
        }

         debug.finish()
    }
}
