use crate::{
    Parser,
    tokenizer::{Token, Tokenizer},
    types::Note,
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};


/// Date encompasses a number of date formats, e.g. approximated, period, phrase and range.
///
/// # Example
///
/// ```rust
/// use gedcom::GedcomDocument;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     1 DATE 2 Oct 2019
///     2 TIME 0:00:00
///     0 @I1@ INDI
///     1 NAME Ancestor
///     1 BIRT
///     2 DATE BEF 1828
///     1 RESI
///     2 PLAC 100 Broadway, New York, NY 10005
///     2 DATE from 1900 to 1905
///     0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
///
/// let head_date = data.header.unwrap().date.unwrap();
/// assert_eq!(head_date.value.unwrap(), "2 Oct 2019");
///
/// let birt_date = data.individuals[0].events[0].date.as_ref().unwrap();
/// assert_eq!(birt_date.value.as_ref().unwrap(), "BEF 1828");
///
/// let resi_date = data.individuals[0].events[1].date.as_ref().unwrap();
/// assert_eq!(resi_date.value.as_ref().unwrap(), "from 1900 to 1905");
/// ```
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Date {
    pub value: Option<String>,
    pub time: Option<String>,
}

impl Date {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> Date {
        let mut date = Date::default();
        date.parse(tokenizer, level);
        date
    }

    /// datetime returns Date and Date.time in a single string.
    pub fn datetime(&self) -> Option<String> {
        match &self.time {
            Some(time) => {
                let mut dt = String::new();
                dt.push_str(self.value.as_ref().unwrap().as_str());
                dt.push_str(" ");
                dt.push_str(&time);
                Some(dt)
            }
            None => None,
        }
    }
}

impl Parser for Date {
    /// parse handles the DATE tag
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
                    "TIME" => self.time = Some(tokenizer.take_line_value()),
                    _ => panic!("{} unhandled DATE tag: {}", tokenizer.debug(), tag),
                },
                Token::Level(_) => tokenizer.next_token(),
                _ => panic!("Unexpected DATE token: {:?}", tokenizer.current_token),
            }
        }
    }
}

/// ChangeDate is intended to only record the last change to a record. Some systems may want to
/// manage the change process with more detail, but it is sufficient for GEDCOM purposes to
/// indicate the last time that a record was modified.
///
/// # Example
/// ```
/// use gedcom::GedcomDocument;
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     2 FORM LINEAGE-LINKED\n\
///     0 @MEDIA1@ OBJE\n\
///     1 FILE /home/user/media/file_name.bmp\n\
///     1 CHAN 
///     2 DATE 1 APR 1998
///     3 TIME 12:34:56.789
///     2 NOTE A note
///     0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
/// assert_eq!(data.multimedia.len(), 1);
///
/// let obje = &data.multimedia[0];
///
/// let chan = obje.change_date.as_ref().unwrap();
/// let date = chan.date.as_ref().unwrap();
/// assert_eq!(date.value.as_ref().unwrap(), "1 APR 1998");
/// assert_eq!(date.time.as_ref().unwrap(), "12:34:56.789");
///
/// let chan_note = chan.note.as_ref().unwrap();
/// assert_eq!(chan_note.value.as_ref().unwrap(), "A note");
/// ```
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ChangeDate {
    pub value: Option<String>,
    pub date: Option<Date>,
    pub note: Option<Note>,
}

impl ChangeDate {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8) -> ChangeDate {
        let mut date = ChangeDate::default();
        date.parse(tokenizer, level);
        date
    }
}

impl Parser for ChangeDate {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        tokenizer.next_token();

        loop {
            if let Token::Level(cur_level) = tokenizer.current_token {
                if cur_level <= level {
                    break;
                }
                tokenizer.next_token();
                match &tokenizer.current_token {
                    Token::Tag(tag) => match tag.as_str() {
                        "DATE" => self.date = Some(Date::new(tokenizer, level + 1)),
                        "NOTE" => self.note = Some(Note::new(tokenizer, level + 1)),
                        _ => panic!("{} unhandled ChangeDate tag: {}", tokenizer.debug(), tag),
                    },
                    Token::Level(_) => tokenizer.next_token(),
                    _ => panic!("Unexpected ChangeDate token: {:?}", tokenizer.current_token),
                }
            }
        }
    }
}
