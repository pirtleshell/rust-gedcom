pub enum EventType {
    Birth,
    Death,
    Marriage,
    Misc(String),
}

pub struct Event {
    r#type: EventType,
    place: Option<String>,
    date: Option<String>,
}
