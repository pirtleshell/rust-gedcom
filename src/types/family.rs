use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
    types::{event::HasEvents, Event, Note},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

/// Family fact, representing a relationship between `Individual`s
///
/// This data representation understands that HUSB & WIFE are just poorly-named
/// pointers to individuals. no gender "validating" is done on parse.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Family {
    pub xref: Option<Xref>,
    pub individual1: Option<Xref>, // mapped from HUSB
    pub individual2: Option<Xref>, // mapped from WIFE
    pub children: Vec<Xref>,
    pub num_children: Option<u8>,
    events: Vec<Event>,
}

impl Family {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> Family {
        let mut fam = Family::default();
        fam.xref = xref;
        fam.children = Vec::new();
        fam.events = Vec::new();
        fam.parse(tokenizer, level);
        fam
    }

    pub fn set_individual1(&mut self, xref: Xref) {
        match self.individual1 {
            Some(_) => panic!("First individual of family already exists."),
            None => self.individual1 = Some(xref),
        };
    }

    pub fn set_individual2(&mut self, xref: Xref) {
        match self.individual2 {
            Some(_) => panic!("Second individual of family already exists."),
            None => self.individual2 = Some(xref),
        };
    }

    pub fn add_child(&mut self, xref: Xref) {
        self.children.push(xref);
    }
}

impl Parser for Family {
    /// parse handles FAM top-level tag
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip over FAM tag name
        tokenizer.next_token();

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }

            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "MARR" => self.add_event(Event::new(tokenizer, level + 1, "MARR")),
                    "HUSB" => self.set_individual1(tokenizer.take_line_value()),
                    "WIFE" => self.set_individual2(tokenizer.take_line_value()),
                    "CHIL" => self.add_child(tokenizer.take_line_value()),
                    _ => panic!("{} Unhandled Family Tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Family Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

impl HasEvents for Family {
    fn add_event(&mut self, event: Event) -> () {
        let event_type = &event.event;
        for e in &self.events {
            if &e.event == event_type {
                panic!("Family already has a {:?} event", e.event);
            }
        }
        self.events.push(event);
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

/// ChildToFamilyLink ...TODO
///
/// # Example
///
/// ```
/// use gedcom::GedcomRecord;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     2 FORM LINEAGE-LINKED\n\
///     0 @PERSON1@ INDI\n\
///     1 NAME given name /surname/jr.
///     1 BIRT
///     2 DATE 31 DEC 1997
///     2 PLAC The place
///     2 FAMC @PARENTS@
///     0 TRLR";
///
/// let mut ged = GedcomRecord::new(sample.chars());
/// let data = ged.parse_record();
///
/// assert_eq!(data.individuals[0].events[0].child_to_family_link.as_ref().unwrap().xref.as_ref().unwrap(), "@PARENTS@");
///
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ChildToFamilyLink {
    pub xref: Option<String>,
    pub pedigree_linkage_type: Option<String>,
    pub child_linkage_status: Option<String>,
    pub note: Option<Note>,
}

impl ChildToFamilyLink {
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> ChildToFamilyLink {
        let mut famc = ChildToFamilyLink {
            xref: None,
            pedigree_linkage_type: None,
            child_linkage_status: None,
            note: None,
        };
        famc.parse(tokenizer, level);
        famc
    }
}

impl Parser for ChildToFamilyLink {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.xref = Some(tokenizer.take_line_value());

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }

            tokenizer.next_token();

            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PEDI" => self.pedigree_linkage_type = Some(tokenizer.take_line_value()),
                    "STAT" => self.child_linkage_status = Some(tokenizer.take_line_value()),
                    "NOTE" => self.note = Some(Note::new(tokenizer, level + 1)),
                    _ => panic!(
                        "{} unhandled ChildToFamilyLink tag: {}",
                        tokenizer.debug(),
                        tag
                    ),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "Unhandled ChildToFamilyLink Token: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}
