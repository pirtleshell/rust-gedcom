use crate::{
    tokenizer::{Token, Tokenizer},
    types::{
        event::HasEvents, Event, MultimediaRecord, Note, SourceCitation, UserDefinedData, Xref,
    },
    Parser,
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// A Person within the family tree
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Individual {
    pub xref: Option<Xref>,
    pub name: Option<Name>,
    pub sex: Option<Gender>,
    pub families: Vec<FamilyLink>,
    pub custom_data: Vec<UserDefinedData>,
    pub source: Vec<SourceCitation>,
    pub multimedia: Vec<MultimediaRecord>,
    pub events: Vec<Event>,
    pub last_updated: Option<String>,
}

impl Individual {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> Individual {
        let mut indi = Individual {
            xref,
            name: None,
            sex: None,
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

    pub fn add_custom_data(&mut self, data: UserDefinedData) {
        self.custom_data.push(data)
    }

    pub fn add_source_citation(&mut self, sour: SourceCitation) {
        self.source.push(sour);
    }

    pub fn add_multimedia(&mut self, multimedia: MultimediaRecord) {
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

impl Parser for Individual {
    /// parse handles the INDI top-level tag
    fn parse(&mut self, tokenizer: &mut crate::tokenizer::Tokenizer, level: u8) {
        // skip over INDI tag name
        tokenizer.next_token();

        while tokenizer.current_token != Token::Level(level) {
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "NAME" => self.name = Some(Name::new(tokenizer, level + 1)),
                    "SEX" => self.sex = Some(Gender::new(tokenizer, level + 1)),
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
                        self.last_updated = Some(tokenizer.take_line_value());
                    }
                    "SOUR" => {
                        self.add_source_citation(SourceCitation::new(tokenizer, level + 1));
                    }
                    // TODO handle xref
                    "OBJE" => {
                        self.add_multimedia(MultimediaRecord::new(tokenizer, level + 1, None))
                    }
                    _ => panic!("{} Unhandled Individual Tag: {}", tokenizer.debug(), tag),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    self.add_custom_data(tokenizer.parse_custom_tag(tag_clone))
                }
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Individual Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

/// GenderType is a set of enumerated values that indicate the sex of an individual at birth. See
/// 5.5 specification, p. 61; https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#SEX
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum GenderType {
    /// Tag 'X'
    Male,
    /// TAG 'M'
    Female,
    /// Tag 'X'; "Does not fit the typical definition of only Male or only Female"
    Nonbinary,
    /// Tag 'U'; "Cannot be determined from available sources"
    Unknown,
}

impl GenderType {
    pub fn get_str(&self) -> &str {
        match self {
            GenderType::Male => "M",
            GenderType::Female => "F",
            GenderType::Nonbinary => "X",
            GenderType::Unknown => "U",
        }
    }
}

/// Gender (tag: SEX); This can describe an individual’s reproductive or sexual anatomy at birth.
/// Related concepts of gender identity or sexual preference are not currently given their own tag.
/// Cultural or personal gender preference may be indicated using the FACT tag. See
/// https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#SEX
///
/// # Example
///
/// ```rust
/// use gedcom::GedcomDocument;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     0 @PERSON1@ INDI\n\
///     1 SEX M
///     2 FACT A fact about an individual's gen
///     3 CONC der
///     2 SOUR @CITATION1@
///     3 PAGE Page
///     4 CONC : 132
///     3 _MYOWNTAG This is a non-standard tag. Not recommended but allowed
///     0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
///
/// let sex = data.individuals[0].sex.as_ref().unwrap();
/// assert_eq!(sex.value.get_str(), "M");
/// assert_eq!(sex.fact.as_ref().unwrap(), "A fact about an individual's gender");
/// assert_eq!(sex.sources[0].xref, "@CITATION1@");
/// assert_eq!(sex.sources[0].page.as_ref().unwrap(), "Page: 132");
/// assert_eq!(sex.sources[0].custom_data[0].tag, "_MYOWNTAG");
/// assert_eq!(sex.sources[0].custom_data[0].value, "This is a non-standard tag. Not recommended but allowed");
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Gender {
    pub value: GenderType,
    pub fact: Option<String>,
    pub sources: Vec<SourceCitation>,
    pub custom_data: Vec<UserDefinedData>,
}

impl Gender {
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Gender {
        let mut sex = Gender {
            value: GenderType::Unknown,
            fact: None,
            sources: Vec::new(),
            custom_data: Vec::new(),
        };
        sex.parse(tokenizer, level);
        sex
    }

    pub fn add_source_citation(&mut self, sour: SourceCitation) {
        self.sources.push(sour);
    }

    pub fn add_custom_data(&mut self, data: UserDefinedData) {
        self.custom_data.push(data)
    }
}

impl Parser for Gender {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        tokenizer.next_token();

        if let Token::LineValue(gender_string) = &tokenizer.current_token {
            self.value = match gender_string.as_str() {
                "M" => GenderType::Male,
                "F" => GenderType::Female,
                "X" => GenderType::Nonbinary,
                "U" => GenderType::Unknown,
                _ => panic!(
                    "{} Unknown gender value {} ({})",
                    tokenizer.debug(),
                    gender_string,
                    level
                ),
            };
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
                    "FACT" => self.fact = Some(tokenizer.take_continued_text(level + 1)),
                    "SOUR" => self.add_source_citation(SourceCitation::new(tokenizer, level + 1)),
                    _ => panic!("{}, Unhandled Gender tag: {}", tokenizer.debug(), tag),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    self.add_custom_data(tokenizer.parse_custom_tag(tag_clone));
                }
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "{}, Unhandled Gender token: {:?}",
                    tokenizer.debug(),
                    tokenizer.current_token
                ),
            }
        }
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
        let xref = tokenizer.take_line_value();
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

impl Parser for FamilyLink {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "PEDI" => self.set_pedigree(tokenizer.take_line_value().as_str()),
                    _ => panic!("{} Unhandled FamilyLink Tag: {}", tokenizer.debug(), tag),
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
    pub note: Option<Note>,
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
            note: None,
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

impl Parser for Name {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(tokenizer.take_line_value());

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "GIVN" => self.given = Some(tokenizer.take_line_value()),
                    "NPFX" => self.prefix = Some(tokenizer.take_line_value()),
                    "NSFX" => self.suffix = Some(tokenizer.take_line_value()),
                    "SPFX" => self.surname_prefix = Some(tokenizer.take_line_value()),
                    "SURN" => self.surname = Some(tokenizer.take_line_value()),
                    "SOUR" => self.add_source_citation(SourceCitation::new(tokenizer, level + 1)),
                    "NOTE" => self.note = Some(Note::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled Name Tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Name Token: {:?}", tokenizer.current_token),
            }
        }
    }
}
