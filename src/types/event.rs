#[derive(Debug)]
pub enum EventType {
    Birth,
    Death,
    Marriage,
}

#[derive(Debug)]
pub struct Event {
    pub r#type: EventType,
    pub place: Option<String>,
    pub date: Option<String>,
}

impl Event {
    pub fn new(etype: EventType) -> Event {
        Event {
            r#type: etype,
            place: None,
            date: None,
        }
    }
}
