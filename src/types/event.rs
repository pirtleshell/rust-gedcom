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
pub enum Event {
    Adoption,
    AdultChristening,
    Baptism,
    BarMitzvah,
    BasMitzvah,
    Birth,
    Blessing,
    Burial,
    Census,
    Christening,
    Confirmation,
    Cremation,
    Death,
    Emigration,
    Event,
    FirstCommunion,
    Graduation,
    Immigration,
    Marriage,
    Naturalization,
    Ordination,
    Probate,
    Probjate,
    Residence,
    Retired,
    Will,
    // "Other" is used to construct an event without requiring an explicit event type
    Other,
    SourceData(String),
}

impl ToString for Event {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

/// EventDetail is a thing that happens on a specific date. Use the date form 'BET date AND date'
/// to indicate that an event took place at some time between two dates. Resist the temptation to
/// use a 'FROM date TO date' form in an event structure. If the subject of your recording occurred
/// over a period of time, then it is probably not an event, but rather an attribute or fact. The
/// EVEN tag in this structure is for recording general events that are not specified in the
/// specification. The event indicated by this general EVEN tag is defined by the value of the
/// subordinate TYPE tag (event_type). 
///
/// # A Minimal Example
///
/// ```rust
/// use gedcom::GedcomDocument;
/// let sample = "\
///    0 HEAD\n\
///    1 GEDC\n\
///    2 VERS 5.5\n\
///    0 @PERSON1@ INDI
///    1 CENS\n\
///    2 DATE 31 DEC 1997\n\
///    2 PLAC The place\n\
///    2 SOUR @SOURCE1@\n\
///    3 PAGE 42\n\
///    3 DATA\n\
///    4 DATE 31 DEC 1900\n\
///    4 TEXT a sample text\n\
///    5 CONT Sample text continued here. The word TE\n\
///    5 CONC ST should not be broken!\n\
///    3 QUAY 3\n\
///    3 NOTE A note\n\
///    4 CONT Note continued here. The word TE\n\
///    4 CONC ST should not be broken!\n\
///    2 NOTE CENSUS event note (the event of the periodic count of the population for a designated locality, such as a national or state Census)\n\
///    3 CONT Note continued here. The word TE\n\
///    3 CONC ST should not be broken!\n\
///    0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
///
/// let event = data.individuals[0].events[0].event.to_string();
/// assert_eq!(event, "Census");
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct EventDetail {
    pub event: Event,
    pub value: Option<String>,
    pub date: Option<Date>,
    pub place: Option<String>,
    pub note: Option<Note>,
    pub child_to_family_link: Option<FamilyLink>,
    /// event_type handles the TYPE tag, a descriptive word or phrase used to further classify the
    /// parent event or attribute tag. This should be used whenever either of the generic EVEN or
    /// FACT tags are used. T. See GEDCOM 5.5 spec, page 35 and 49.
    pub event_type: Option<String>,
    pub citations: Vec<SourceCitation>,
}

impl EventDetail {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, tag: &str) -> EventDetail {
        let mut event = EventDetail {
            event: Self::from_tag(tag),
            value: None,
            date: None,
            place: None,
            note: None,
            child_to_family_link: None,
            event_type: None,
            citations: Vec::new(),
        };
        event.parse(tokenizer, level);
        event
    }

    /** converts an event to be of type `SourceData` with `value` as the data */
    pub fn with_source_data(&mut self, value: String) {
        self.event = Event::SourceData(value);
    }

    pub fn from_tag(tag: &str) -> Event {
        match tag {
            "ADOP" => Event::Adoption,
            "BAPM" => Event::Baptism,
            "BARM" => Event::BarMitzvah,
            "BASM" => Event::BasMitzvah,
            "BIRT" => Event::Birth,
            "BLES" => Event::Blessing,
            "BURI" => Event::Burial,
            "CENS" => Event::Census,
            "CHR" => Event::Christening,
            "CHRA" => Event::AdultChristening,
            "CONF" => Event::Confirmation,
            "CREM" => Event::Cremation,
            "DEAT" => Event::Death,
            "EMIG" => Event::Emigration,
            "EVEN" => Event::Event,
            "FCOM" => Event::FirstCommunion,
            "GRAD" => Event::Graduation,
            "IMMI" => Event::Immigration,
            "MARR" => Event::Marriage,
            "NATU" => Event::Naturalization,
            "ORDN" => Event::Ordination,
            "PROB" => Event::Probate,
            "RESI" => Event::Residence,
            "RETI" => Event::Retired,
            "WILL" => Event::Will,
            "OTHER" => Event::Other,
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

impl std::fmt::Debug for EventDetail {
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
    fn add_event(&mut self, event: EventDetail) -> ();
    fn events(&self) -> Vec<EventDetail>;
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

impl Parser for EventDetail {
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
                    "TYPE" => self.event_type = Some(tokenizer.take_line_value()),
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
