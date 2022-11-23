use crate::{
    parser::Parser,
    tokenizer::{Token, Tokenizer},
    types::{Note, SourceCitation, Xref},
};

use super::ChangeDate;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]

/// The multimedia record refers to 1 or more external digital files, and may provide some
/// additional information about the files and the media they encode.
///
/// The file reference can occur more than once to group multiple files together. Grouped files
/// should each pertain to the same context. For example, a sound clip and a photo both of the same
/// event might be grouped in a single OBJE.
///
/// The change and creation dates should be for the OBJE record itself, not the underlying files.
///
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#MULTIMEDIA_RECORD.
pub struct MultimediaRecord {
    /// Optional reference to link to this submitter
    pub xref: Option<Xref>,
    pub file: Option<MultimediaFileRefn>,
    /// The 5.5 spec, page 26, shows FORM as a sub-structure of FILE, but the struct appears as a
    /// sibling in an Ancestry.com export.
    pub form: Option<MultimediaFormat>,
    /// The 5.5 spec, page 26, shows TITL as a sub-structure of FILE, but the struct appears as a
    /// sibling in an Ancestry.com export.
    pub title: Option<String>,
    pub user_reference_number: Option<UserReferenceNumber>,
    pub automated_record_id: Option<String>,
    pub source_citation: Option<SourceCitation>,
    pub change_date: Option<ChangeDate>,
    pub note_structure: Option<Note>,
}

impl MultimediaRecord {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> MultimediaRecord {
        let mut obje = MultimediaRecord {
            xref,
            file: None,
            form: None,
            title: None,
            user_reference_number: None,
            automated_record_id: None,
            source_citation: None,
            change_date: None,
            note_structure: None,
        };
        obje.parse(tokenizer, level);
        obje
    }
}

impl Parser for MultimediaRecord {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip current line
        tokenizer.next_token();
        loop {
            if let Token::Level(curl_level) = tokenizer.current_token {
                if curl_level <= level {
                    break;
                }
            }
            tokenizer.next_token();
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "FILE" => self.file = Some(MultimediaFileRefn::new(tokenizer, level + 1)),
                    "FORM" => self.form = Some(MultimediaFormat::new(tokenizer, level + 1)),
                    "TITL" => self.title = Some(tokenizer.take_line_value()),
                    "REFN" => {
                        self.user_reference_number =
                            Some(UserReferenceNumber::new(tokenizer, level + 1))
                    }
                    "RIN" => self.automated_record_id = Some(tokenizer.take_line_value()),
                    "NOTE" => self.note_structure = Some(Note::new(tokenizer, level + 1)),
                    "SOUR" => {
                        self.source_citation = Some(SourceCitation::new(tokenizer, level + 1))
                    }
                    "CHAN" => self.change_date = Some(ChangeDate::new(tokenizer, level + 1)),
                    _ => panic!("{} Unhandled Multimedia Tag: {}", tokenizer.debug(), tag),
                },
                _ => panic!("Unhandled Multimedia Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// MultimediaLink
pub struct MultimediaLink {
    /// Optional reference to link to this submitter
    pub xref: Option<Xref>,
    pub file: Option<MultimediaFileRefn>,
    /// The 5.5 spec, page 26, shows FORM as a sub-structure of FILE, but the struct appears as a
    /// sibling in an Ancestry.com export.
    pub form: Option<MultimediaFormat>,
    /// The 5.5 spec, page 26, shows TITL as a sub-structure of FILE, but the struct appears as a
    /// sibling in an Ancestry.com export.
    pub title: Option<String>,
}

impl MultimediaLink {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> MultimediaLink {
        let mut obje = MultimediaLink {
            xref,
            file: None,
            form: None,
            title: None,
        };
        obje.parse(tokenizer, level);
        obje
    }
}

impl Parser for MultimediaLink {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip current line
        tokenizer.next_token();
        loop {
            if let Token::Level(curl_level) = tokenizer.current_token {
                if curl_level <= level {
                    break;
                }
            }
            tokenizer.next_token();
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "FILE" => self.file = Some(MultimediaFileRefn::new(tokenizer, level + 1)),
                    "FORM" => self.form = Some(MultimediaFormat::new(tokenizer, level + 1)),
                    "TITL" => self.title = Some(tokenizer.take_line_value()),
                    _ => panic!("{} Unhandled Multimedia Tag: {}", tokenizer.debug(), tag),
                },
                _ => panic!("Unhandled Multimedia Token: {:?}", tokenizer.current_token),
            }
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]

/// A complete local or remote file reference to the auxiliary data to be linked to the GEDCOM
/// context. Remote reference would include a network address where the multimedia data may
/// be obtained.
pub struct MultimediaFileRefn {
    pub value: Option<String>,
    pub title: Option<String>,
    pub form: Option<MultimediaFormat>,
}

impl MultimediaFileRefn {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> MultimediaFileRefn {
        let mut file = MultimediaFileRefn::default();
        file.parse(tokenizer, level);
        file
    }
}

impl Parser for MultimediaFileRefn {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(tokenizer.take_line_value());
        loop {
            if let Token::Level(curl_level) = &tokenizer.current_token {
                if curl_level <= &level {
                    break;
                }
            }
            tokenizer.next_token();
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "TITL" => self.title = Some(tokenizer.take_line_value()),
                    "FORM" => self.form = Some(MultimediaFormat::new(tokenizer, level + 1)),
                    _ => panic!(
                        "{} Unhandled MultimediaFileRefn Tag: {}",
                        tokenizer.debug(),
                        tag
                    ),
                },
                _ => panic!(
                    "Unhandled MultimediaFileRefn Token: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]

/// Indicates the format of the multimedia data associated with the specific GEDCOM context. This
/// allows processors to determine whether they can process the data object. Any linked files should
/// contain the data required, in the indicated format, to process the file data.
///
/// NOTE: The 5.5 spec lists the following seven formats [ bmp | gif | jpg | ole | pcx | tif | wav ].
/// However, we're leaving this open for emerging formats, Option<String>.
pub struct MultimediaFormat {
    pub value: Option<String>,
    pub source_media_type: Option<String>,
}

impl MultimediaFormat {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> MultimediaFormat {
        let mut form = MultimediaFormat::default();
        form.parse(tokenizer, level);
        form
    }
}

impl Parser for MultimediaFormat {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(tokenizer.take_line_value());
        loop {
            if let Token::Level(curl_level) = &tokenizer.current_token {
                if curl_level <= &level {
                    break;
                }
            }
            tokenizer.next_token();
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "TYPE" => self.source_media_type = Some(tokenizer.take_line_value()),
                    _ => panic!("{} Unhandled MultimediaFormat Tag: {}", tokenizer.debug(), tag),
                },
                _ => panic!(
                    "Unhandled MultimediaFormat Token: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]

/// A user-defined number or text that the submitter uses to identify this record. For instance, it
/// may be a record number within the submitter's automated or manual system, or it may be a page
/// and position number on a pedigree chart.
pub struct UserReferenceNumber {
    /// line value
    pub value: Option<String>,
    /// A user-defined definition of the USER_REFERENCE_NUMBER.
    pub user_reference_type: Option<String>,
}

impl UserReferenceNumber {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> UserReferenceNumber {
        let mut refn = UserReferenceNumber::default();
        refn.parse(tokenizer, level);
        refn
    }
}

impl Parser for UserReferenceNumber {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        self.value = Some(tokenizer.take_line_value());

        loop {
            if let Token::Level(curl_level) = &tokenizer.current_token {
                if curl_level <= &level {
                    break;
                }
            }
            match &tokenizer.current_token {
                Token::Tag(tag) => match tag.as_str() {
                    "TYPE" => self.user_reference_type = Some(tokenizer.take_line_value()),
                    _ => panic!(
                        "{} Unhandled UserReferenceNumber Tag: {}",
                        tokenizer.debug(),
                        tag
                    ),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!(
                    "Unhandled UserReferenceNumber Token: {:?}",
                    tokenizer.current_token
                ),
            }
        }
    }
}
