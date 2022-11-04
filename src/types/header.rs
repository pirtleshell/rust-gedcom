use crate::util::dbg;
use crate::{
    parser::Parse,
    tokenizer::{Token, Tokenizer},
    types::{Copyright, Corporation, Date, Note},
    util::{parse_custom_tag, take_line_value},
};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use super::CustomData;

/// Header (tag: HEAD) containing GEDCOM metadata.
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEADER
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Header {
    /// tag: GEDC
    pub gedcom: Option<GedcomDoc>,
    /// tag: CHAR
    pub encoding: Option<Encoding>,
    /// tag: SOUR
    pub source: Option<HeadSour>,
    /// tag: DEST, an identifier for the system expected to receive this document.
    /// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#DEST
    pub destination: Option<String>,
    /// tag: DATE
    pub date: Option<Date>,
    /// tag: SUBM See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#SUBM
    pub submitter_tag: Option<String>,
    /// tag: SUBN
    pub submission_tag: Option<String>,
    /// tag: COPR
    pub copyright: Option<Copyright>,
    /// tag: LANG (HEAD-LANG), a default language which may be used to interpret any Text-typed
    /// payloads that lack a specific language tag from a LANG structure. An application may choose
    /// to use a different default based on its knowledge of the language preferences of the user.
    /// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEAD-LANG
    pub language: Option<String>,
    /// tag: FILE, the name of the GEDCOM transmission file. If the file name includes a file
    /// extension it must be shown in the form (filename.ext). See Gedcom 5.5.1 specification, p. 50.
    pub filename: Option<String>,
    /// tag: NOTE
    pub note: Option<Note>,
    /// tag: PLAC
    pub place: Option<HeadPlac>,
    pub custom_data: Vec<CustomData>,
}

impl Header {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Header {
        let mut header = Header::default();
        header.parse(tokenizer, level);
        header
    }

    pub fn add_custom_data(&mut self, data: CustomData) {
        self.custom_data.push(data)
    }
}

impl Parse for Header {
    /// Parses HEAD top-level tag. See
    /// https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEADER
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // let mut head = Header::default();

        // skip over HEAD tag name
        tokenizer.next_token();

        while tokenizer.current_token != Token::Level(level) {
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "GEDC" => self.gedcom = Some(GedcomDoc::new(tokenizer, 1)),
                    "SOUR" => self.source = Some(HeadSour::new(tokenizer, 1)),
                    "DEST" => self.destination = Some(take_line_value(tokenizer)),
                    "DATE" => self.date = Some(Date::new(tokenizer, 1)),
                    "SUBM" => self.submitter_tag = Some(take_line_value(tokenizer)),
                    "SUBN" => self.submission_tag = Some(take_line_value(tokenizer)),
                    "FILE" => self.filename = Some(take_line_value(tokenizer)),
                    "COPR" => self.copyright = Some(Copyright::new(tokenizer, 1)),
                    "CHAR" => self.encoding = Some(Encoding::new(tokenizer, 1)),
                    "LANG" => self.language = Some(take_line_value(tokenizer)),
                    "NOTE" => self.note = Some(Note::new(tokenizer, 1)),
                    "PLAC" => self.place = Some(HeadPlac::new(tokenizer, 1)),
                    _ => panic!("{} Unhandled Header Tag: {}", dbg(tokenizer), tag),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    self.add_custom_data(parse_custom_tag(tokenizer, tag_clone))
                }
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unhandled Header Token: {:?}", &tokenizer.current_token),
            }
        }
    }
}

/// GedcomDoc (tag: GEDC) is a container for information about the entire document. It is
/// recommended that applications write GEDC with its required subrecord VERS as the first
/// substructure of a HEAD. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#GEDC
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct GedcomDoc {
    /// tag: VERS
    pub version: Option<String>,
    /// tag: FORM; see Gedcom 5.5.1 specification, p. 50
    pub form: Option<String>,
}

impl GedcomDoc {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> GedcomDoc {
        let mut gedc = GedcomDoc::default();
        gedc.parse(tokenizer, level);
        gedc
    }
}

impl Parse for GedcomDoc {
    /// parse handles parsing GEDC tag
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip GEDC tag
        tokenizer.next_token();

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }

            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "VERS" => self.version = Some(take_line_value(tokenizer)),
                    // this is the only value that makes sense. warn them otherwise.
                    "FORM" => {
                        let form = take_line_value(tokenizer);
                        if &form.to_uppercase() != "LINEAGE-LINKED" {
                            println!(
                                "WARNING: Unrecognized GEDCOM form. Expected LINEAGE-LINKED, found {}"
                            , form);
                        }
                        self.form = Some(form);
                    }
                    _ => panic!("{} Unhandled GEDC Tag: {}", dbg(&tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "{} Unexpected GEDC Token: {:?}",
                    dbg(&tokenizer),
                    &tokenizer.current_token
                ),
            }
        }
    }
}

/// Encoding (tag: CHAR) is a code value that represents the character set to be used to
/// interpret this data. See Gedcom 5.5.1 specification, p. 44
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Encoding {
    pub value: Option<String>,
    /// tag: VERS
    pub version: Option<String>,
}

impl Encoding {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Encoding {
        let mut chars = Encoding::default();
        chars.parse(tokenizer, level);
        chars
    }
}

impl Parse for Encoding {
    /// parse handles the parsing of the CHARS tag
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
                    "VERS" => self.version = Some(take_line_value(tokenizer)),
                    _ => panic!("{} Unhandled CHAR Tag: {}", dbg(&tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "{} Unexpected CHAR Token: {:?}",
                    dbg(&tokenizer),
                    &tokenizer.current_token
                ),
            }
        }
    }
}

/// HeadSource (tag: SOUR) is an identifier for the product producing the gedcom data. A
/// registration process for these identifiers existed for a time, but no longer does. If an
/// existing identifier is known, it should be used. Otherwise, a URI owned by the product should
/// be used instead. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEAD-SOUR
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HeadSour {
    pub value: Option<String>,
    /// tag: VERS
    pub version: Option<String>,
    /// tag: NAME
    pub name: Option<String>,
    /// tag: CORP
    pub corporation: Option<Corporation>,
    /// tag: DATA
    pub data: Option<HeadSourData>,
}

impl HeadSour {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> HeadSour {
        let mut head_sour = HeadSour::default();
        head_sour.parse(tokenizer, level);
        head_sour
    }
}

impl Parse for HeadSour {
    /// parse handles the SOUR tag in a header
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
                    "VERS" => self.version = Some(take_line_value(tokenizer)),
                    "NAME" => self.name = Some(take_line_value(tokenizer)),
                    "CORP" => self.corporation = Some(Corporation::new(tokenizer, level + 1)),
                    "DATA" => self.data = Some(HeadSourData::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled CHAR Tag: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unexpected SOUR Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

/// The electronic data source or digital repository from which this dataset was exported. The
/// payload is the name of that source, with substructures providing additional details about the
/// source (not the export). See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEAD-SOUR-DATA
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HeadSourData {
    pub value: Option<String>,
    /// tag: DATE
    pub date: Option<Date>,
    /// tag: COPR
    pub copyright: Option<Copyright>,
}

impl HeadSourData {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> HeadSourData {
        let mut head_sour_data = HeadSourData::default();
        head_sour_data.parse(tokenizer, level);
        head_sour_data
    }
}

impl Parse for HeadSourData {
    /// parse parses the DATA tag
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
                    "DATE" => self.date = Some(Date::new(tokenizer, level + 1)),
                    "COPR" => self.copyright = Some(Copyright::new(tokenizer, level + 1)),
                    _ => panic!("{} unhandled DATA tag in header: {}", dbg(tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "Unhandled SOUR tag in header: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}

/// HeadPlace (tag: PLAC) is is a placeholder for providing a default PLAC.FORM, and must not have
/// a payload. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEAD-PLAC
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HeadPlac {
    /// form (tag: FORM) is a comma-separated list of jurisdictional titles (e.g. City, County,
    /// State, Country). It has the same number of elements and in the same order as the PLAC
    /// structure. As with PLAC, this shall be ordered from lowest to highest jurisdiction.
    /// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#PLAC-FORM
    pub form: Vec<String>,
}

impl HeadPlac {
    pub fn push_jurisdictional_title(&mut self, title: String) {
        self.form.push(title);
    }

    // Adhering to "lowest to highest jurisdiction" is the responsibility of the
    // Gedcom author, but methods for reordering elements might still be useful.
    pub fn insert_jurisdictional_title(&mut self, index: usize, title: String) {
        self.form.insert(index, title);
    }

    pub fn remove_jurisdictional_title(&mut self, index: usize) {
        self.form.remove(index);
    }
}

impl HeadPlac {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> HeadPlac {
        let mut head_plac = HeadPlac::default();
        head_plac.parse(tokenizer, level);
        head_plac
    }
}

impl Parse for HeadPlac {
    /// parse handles the PLAC tag when present in header
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {

        // In the header, PLAC should have no payload. See
        // https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEAD-PLAC
        tokenizer.next_token();
        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "FORM" => {
                        let form = take_line_value(tokenizer);
                        let jurisdictional_titles = form.split(",");

                        for t in jurisdictional_titles {
                            let v = t.trim();
                            self.push_jurisdictional_title(v.to_string());
                        }
                    }
                    _ => panic!("{} Unhandled PLAC tag in header: {}", dbg(&tokenizer), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "Unhandled PLAC tag in header: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}
