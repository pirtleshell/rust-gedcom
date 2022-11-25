use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
    types::{Copyright, Corporation, Date, Note},
};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use super::UserDefinedData;

/// Header (tag: HEAD) containing GEDCOM metadata.
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEADER
///
/// # Example
///
/// ```
/// use gedcom::GedcomRecord;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     1 DEST Destination of transmission\n\
///     1 DATE 1 JAN 1998\n\
///     2 TIME 13:57:24.80\n\
///     1 SUBM @SUBMITTER@\n\
///     1 SUBN @SUBMISSION@\n\
///     1 FILE ALLGED.GED\n\
///     1 COPR (C) 1997-2000 by H. Eichmann.\n\
///     2 CONT You can use and distribute this file freely as long as you do not charge for it.\n\
///     1 LANG language
///     0 TRLR";
///
/// let mut parser = GedcomRecord::new(sample.chars());
/// let data = parser.parse_record();
///
/// let header = data.header.unwrap();
/// assert_eq!(header.gedcom.unwrap().version.unwrap(), "5.5");
///
/// assert_eq!(
///     header.destination.unwrap(),
///     "Destination of transmission"
/// );
///
/// let date = header.date.unwrap();
/// assert_eq!(date.value.unwrap(), "1 JAN 1998");
/// assert_eq!(date.time.unwrap(), "13:57:24.80");
///
/// let subm = header.submission_tag.unwrap();
/// assert_eq!(subm, "@SUBMISSION@");
///
/// let file = header.filename.unwrap();
/// assert_eq!(file, "ALLGED.GED");
///
/// let copr = header.copyright.unwrap();
/// assert_eq!(copr.value.unwrap(), "(C) 1997-2000 by H. Eichmann.");
/// assert_eq!(
///     copr.continued.unwrap(),
///     "You can use and distribute this file freely as long as you do not charge for it."
/// );
///
/// let lang = header.language.unwrap();
/// assert_eq!(lang.as_str(), "language");
/// ```
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
    pub custom_data: Vec<UserDefinedData>,
}

impl Header {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Header {
        let mut header = Header::default();
        header.parse(tokenizer, level);
        header
    }

    pub fn add_custom_data(&mut self, data: UserDefinedData) {
        self.custom_data.push(data)
    }
}

impl Parser for Header {
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
                    "DEST" => self.destination = Some(tokenizer.take_line_value()),
                    "DATE" => self.date = Some(Date::new(tokenizer, 1)),
                    "SUBM" => self.submitter_tag = Some(tokenizer.take_line_value()),
                    "SUBN" => self.submission_tag = Some(tokenizer.take_line_value()),
                    "FILE" => self.filename = Some(tokenizer.take_line_value()),
                    "COPR" => self.copyright = Some(Copyright::new(tokenizer, 1)),
                    "CHAR" => self.encoding = Some(Encoding::new(tokenizer, 1)),
                    "LANG" => self.language = Some(tokenizer.take_line_value()),
                    "NOTE" => self.note = Some(Note::new(tokenizer, 1)),
                    "PLAC" => self.place = Some(HeadPlac::new(tokenizer, 1)),
                    _ => panic!("{} Unhandled Header Tag: {}", tokenizer.debug(), tag),
                },
                Token::CustomTag(tag) => {
                    let tag_clone = tag.clone();
                    self.add_custom_data(tokenizer.parse_custom_tag(tag_clone))
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
///     0 TRLR";
///
/// let mut ged = GedcomRecord::new(sample.chars());
/// let data = ged.parse_record();
///
/// let head_gedc = data.header.unwrap().gedcom.unwrap();
/// assert_eq!(head_gedc.version.unwrap(), "5.5");
/// assert_eq!(head_gedc.form.unwrap(), "LINEAGE-LINKED");
/// ```
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

impl Parser for GedcomDoc {
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
                    "VERS" => self.version = Some(tokenizer.take_line_value()),
                    // this is the only value that makes sense. warn them otherwise.
                    "FORM" => {
                        let form = tokenizer.take_line_value();
                        if &form.to_uppercase() != "LINEAGE-LINKED" {
                            println!(
                                "WARNING: Unrecognized GEDCOM form. Expected LINEAGE-LINKED, found {}"
                            , form);
                        }
                        self.form = Some(form);
                    }
                    _ => panic!("{} Unhandled GEDC Tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "{} Unexpected GEDC Token: {:?}",
                    tokenizer.debug(),
                    &tokenizer.current_token
                ),
            }
        }
    }
}

/// Encoding (tag: CHAR) is a code value that represents the character set to be used to
/// interpret this data. See Gedcom 5.5.1 specification, p. 44
///
/// # Example
///
/// ```
/// use gedcom::GedcomRecord;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     1 CHAR ASCII\n\
///     2 VERS Version number of ASCII (whatever it means)\n\
///     0 TRLR";

/// let mut parser = GedcomRecord::new(sample.chars());
/// let data = parser.parse_record();

/// let h_char = data.header.unwrap().encoding.unwrap();
/// assert_eq!(h_char.value.unwrap(), "ASCII");
/// assert_eq!(
///     h_char.version.unwrap(),
///     "Version number of ASCII (whatever it means)"
/// );
/// ```
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

impl Parser for Encoding {
    /// parse handles the parsing of the CHARS tag
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
                    "VERS" => self.version = Some(tokenizer.take_line_value()),
                    _ => panic!("{} Unhandled CHAR Tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "{} Unexpected CHAR Token: {:?}",
                    tokenizer.debug(),
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
///
/// # Example
///
/// ```
/// use gedcom::GedcomRecord;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     1 SOUR SOURCE_NAME\n\
///     2 VERS Version number of source-program\n\
///     2 NAME Name of source-program\n\
///     0 TRLR";
///
/// let mut parser = GedcomRecord::new(sample.chars());
/// let data = parser.parse_record();
///
/// let sour = data.header.unwrap().source.unwrap();
/// assert_eq!(sour.value.unwrap(), "SOURCE_NAME");
///
/// let vers = sour.version.unwrap();
/// assert_eq!(vers, "Version number of source-program");
///
/// let name = sour.name.unwrap();
/// assert_eq!(name, "Name of source-program");
/// ```
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

impl Parser for HeadSour {
    /// parse handles the SOUR tag in a header
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
                    "VERS" => self.version = Some(tokenizer.take_line_value()),
                    "NAME" => self.name = Some(tokenizer.take_line_value()),
                    "CORP" => self.corporation = Some(Corporation::new(tokenizer, level + 1)),
                    "DATA" => self.data = Some(HeadSourData::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled CHAR Tag: {}", tokenizer.debug(), tag),
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
///
/// # Example
///
/// ```
/// use gedcom::GedcomRecord;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     1 SOUR SOURCE_NAME\n\
///     2 DATA Name of source data\n\
///     3 DATE 1 JAN 1998\n\
///     3 COPR Copyright of source data\n\
///     0 TRLR";
///
/// let mut parser = GedcomRecord::new(sample.chars());
/// let data = parser.parse_record();
///
/// let sour = data.header.unwrap().source.unwrap();
/// assert_eq!(sour.value.unwrap(), "SOURCE_NAME");
///
/// let sour_data = sour.data.unwrap();
/// assert_eq!(sour_data.value.unwrap(), "Name of source data");
/// assert_eq!(sour_data.date.unwrap().value.unwrap(), "1 JAN 1998");
/// assert_eq!(
///     sour_data.copyright.unwrap().value.unwrap(),
///     "Copyright of source data"
/// );
/// ```
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

impl Parser for HeadSourData {
    /// parse parses the DATA tag
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
                    "DATE" => self.date = Some(Date::new(tokenizer, level + 1)),
                    "COPR" => self.copyright = Some(Copyright::new(tokenizer, level + 1)),
                    _ => panic!(
                        "{} unhandled DATA tag in header: {}",
                        tokenizer.debug(),
                        tag
                    ),
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
///
/// # Example
///
/// ```
/// use gedcom::GedcomRecord;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     1 PLAC\n\
///     2 FORM City, County, State, Country\n\
///     0 TRLR";
///
/// let mut parser = GedcomRecord::new(sample.chars());
/// let data = parser.parse_record();
///
/// let h_plac = data.header.unwrap().place.unwrap();
/// assert_eq!(h_plac.form[0], "City");
/// assert_eq!(h_plac.form[1], "County");
/// assert_eq!(h_plac.form[2], "State");
/// assert_eq!(h_plac.form[3], "Country");
/// ```
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

impl Parser for HeadPlac {
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
                        let form = tokenizer.take_line_value();
                        let jurisdictional_titles = form.split(",");

                        for t in jurisdictional_titles {
                            let v = t.trim();
                            self.push_jurisdictional_title(v.to_string());
                        }
                    }
                    _ => panic!(
                        "{} Unhandled PLAC tag in header: {}",
                        tokenizer.debug(),
                        tag
                    ),
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
