use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    types::{event::HasEvents, CustomData, Event, Multimedia, SourceCitation},
    util::{dbg, parse_custom_tag, take_line_value},
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type Xref = String;

/// A Person within the family tree
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Individual {
    pub xref: Option<Xref>,
    pub name: Option<Name>,
    pub sex: Gender,
    pub families: Vec<FamilyLink>,
    pub custom_data: Vec<CustomData>,
    pub last_updated: Option<String>,
    pub source: Vec<SourceCitation>,
    pub multimedia: Vec<Multimedia>,
    events: Vec<Event>,
}

impl Individual {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> Individual {
        let mut indi = Individual {
            xref,
            name: None,
            sex: Gender::Unknown,
            events: Vec::new(),
            families: Vec::new(),
            custom_data: Vec::new(),
            last_updated: None,
            source: Vec::new(),
            multimedia: Vec::new(),
        };
        indi.parse(tokenizer, level);
        indi
    }

    pub fn add_family(&mut self, link: FamilyLink) {
        let mut do_add = true;
        let xref = &link.0;
        for FamilyLink(family, _, _) in &self.families {
            if family.as_str() == xref.as_str() {
                do_add = false;
            }
        }
        if do_add {
            self.families.push(link);
        }
    }

    pub fn add_custom_data(&mut self, data: CustomData) {
        self.custom_data.push(data)
    }

    pub fn add_source_citation(&mut self, sour: SourceCitation) {
        self.source.push(sour);
    }

    pub fn add_multimedia(&mut self, multimedia: Multimedia) {
        self.multimedia.push(multimedia);
    }
}

impl HasEvents for Individual {
    fn add_event(&mut self, event: Event) -> () {
        self.events.push(event);
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

impl Parse for Individual {
    /// parse handles the INDI top-level tag
    fn parse(&mut self, tokenizer: &mut crate::tokenizer::Tokenizer, level: u8) {
        // skip over INDI tag name
        tokenizer.next_token();

        while tokenizer.current_token != Token::Level(level) {
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => self.name = Some(Name::new(tokenizer, level + 1)),
                    "SEX" => self.sex = Gender::new(tokenizer, level + 1),
                    "ADOP" | "BIRT" | "BAPM" | "BARM" | "BASM" | "BLES" | "BURI" | "CENS"
                    | "CHR" | "CHRA" | "CONF" | "CREM" | "DEAT" | "EMIG" | "FCOM" | "GRAD"
                    | "IMMI" | "NATU" | "ORDN" | "RETI" | "RESI" | "PROB" | "WILL" | "EVEN"
                    | "MARR" => {
                        let tag_clone = tag.clone();
                        self.add_event(Event::new(tokenizer, level + 1, tag_clone.as_str()));
                    }
                    "FAMC" | "FAMS" => {
                        let tag_clone = tag.clone();
                        self.add_family(FamilyLink::new(tokenizer, level + 1, tag_clone.as_str()));
                    }
                    "CHAN" => {
                        // assuming it always only has a single DATE subtag
                        tokenizer.next_token(); // level
                        tokenizer.next_token(); // DATE tag
                        self.last_updated = Some(take_line_value(tokenizer));
                    }
                    "SOUR" => {
                        self.add_source_citation(SourceCitation::new(tokenizer, level + 1));
                    }
                    // TODO handle xref
                    "OBJE" => self.add_multimedia(Multimedia::new(tokenizer, level + 1, None)),
                    _ => panic!("{} Unhandled Individual Tag: {}", dbg(tokenizer), tag),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    self.add_custom_data(parse_custom_tag(tokenizer, tag_clone))
                }
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Individual Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

/// Gender of an `Individual`
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum Gender {
    Male,
    Female,
    Nonbinary,
    Unknown,
}

impl Gender {
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Gender {
        let mut gender = Gender::Unknown;
        gender.parse(tokenizer, level);
        gender
    }
}

impl Parse for Gender {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        tokenizer.next_token();
        if let Token::LineValue(gender_string) = &tokenizer.current_token {
            *self = match gender_string.as_str() {
                "M" => Gender::Male,
                "F" => Gender::Female,
                "N" => Gender::Nonbinary,
                "U" => Gender::Unknown,
                _ => panic!(
                    "{} Unknown gender value {} ({})",
                    dbg(tokenizer),
                    gender_string,
                    level
                ),
            };
        } else {
            panic!(
                "Expected gender LineValue, found {:?}",
                tokenizer.current_token
            );
        }
        tokenizer.next_token();
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
enum FamilyLinkType {
    Spouse,
    Child,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
enum Pedigree {
    Adopted,
    Birth,
    Foster,
    Sealing,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct FamilyLink(Xref, FamilyLinkType, Option<Pedigree>);

impl FamilyLink {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, tag: &str) -> FamilyLink {
        let xref = take_line_value(tokenizer);
        let link_type = match tag {
            "FAMC" => FamilyLinkType::Child,
            "FAMS" => FamilyLinkType::Spouse,
            _ => panic!("Unrecognized family type tag: {}", tag),
        };
        let mut family_link = FamilyLink(xref, link_type, None);
        family_link.parse(tokenizer, level);
        family_link
    }

    pub fn set_pedigree(&mut self, pedigree_text: &str) {
        self.2 = match pedigree_text.to_lowercase().as_str() {
            "adopted" => Some(Pedigree::Adopted),
            "birth" => Some(Pedigree::Birth),
            "foster" => Some(Pedigree::Foster),
            "sealing" => Some(Pedigree::Sealing),
            _ => panic!("Unrecognized family link pedigree: {}", pedigree_text),
        };
    }
}

impl Parse for FamilyLink {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PEDI" => self.set_pedigree(take_line_value(tokenizer).as_str()),
                    _ => panic!("{} Unhandled FamilyLink Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled FamilyLink Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Name {
    pub value: Option<String>,
    pub given: Option<String>,
    pub surname: Option<String>,
    pub prefix: Option<String>,
    pub surname_prefix: Option<String>,
    pub suffix: Option<String>,
    pub source: Vec<SourceCitation>,
}

impl Name {
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Name {
        let mut name = Name {
            value: None,
            given: None,
            surname: None,
            prefix: None,
            surname_prefix: None,
            suffix: None,
            source: Vec::new(),
        };
        name.parse(tokenizer, level);
        name
    }

    pub fn add_source_citation(&mut self, sour: SourceCitation) {
        self.source.push(sour);
    }
}

impl Parse for Name {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(take_line_value(tokenizer));

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "GIVN" => self.given = Some(take_line_value(tokenizer)),
                    "NPFX" => self.prefix = Some(take_line_value(tokenizer)),
                    "NSFX" => self.suffix = Some(take_line_value(tokenizer)),
                    "SPFX" => self.surname_prefix = Some(take_line_value(tokenizer)),
                    "SURN" => self.surname = Some(take_line_value(tokenizer)),
                    "SOUR" => self.add_source_citation(SourceCitation::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled Name Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Name Token: {:?}", tokenizer.current_token),
            }
        }
    }
}
