use crate::{
    parse_subset,
    tokenizer::Tokenizer,
    types::{ChangeDate, Note, UserDefinedDataset, Xref},
    Parser,
};

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Submission is used by the sending system to send instructions and information to the
/// receiving system. The sending system uses a submission record to send instructions and
/// information to the receiving system. The submission record is also used for communication
/// between Ancestral File download requests and TempleReady. Each GEDCOM transmission file should
/// have only one submission record. Multiple submissions are handled by creating separate GEDCOM
/// transmission files. See GEDCOM 5.5 spec, page 28.
///
/// # Example
///
/// ```rust
/// use gedcom::GedcomDocument;
/// let sample = "\
///    0 HEAD\n\
///    1 GEDC\n\
///    2 VERS 5.5\n\
///    0 @SUBMISSION@ SUBN\n\
///    1 _MYOWNTAG SUBN does not allow NOTE tags :-(( so, here is my not: SUBN seems to be LDS internal data. The sample data I put in here are probably nonsence.\n\
///    1 SUBM @SUBMITTER@\n\
///    1 FAMF NameOfFamilyFile\n\
///    1 TEMP Abreviated temple code\n\
///    1 ANCE 1\n\
///    1 DESC 1\n\
///    1 ORDI yes\n\
///    0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
/// ```
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Submission {
    pub xref: Option<Xref>,
    pub name_of_family_file: Option<String>,
    pub temple_code: Option<String>,
    pub submitter_link: Option<String>,
    pub generations_of_ancestors: Option<String>,
    pub generations_of_descendants: Option<String>,
    pub ordinance_process_flag: Option<String>,
    pub automated_record_id: Option<String>,
    pub note: Option<Note>,
    pub change_date: Option<ChangeDate>,
    pub custom_data: Vec<Box<UserDefinedDataset>>,
}

impl Submission {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, xref: Option<Xref>) -> Submission {
        let mut subn = Submission::default();
        subn.xref = xref;
        subn.parse(tokenizer, level);
        subn
    }
}

impl Parser for Submission {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        tokenizer.next_token();

        let handle_subset = |tag: &str, tokenizer: &mut Tokenizer| match tag {
            "ANCE" => self.generations_of_ancestors = Some(tokenizer.take_line_value()),
            "DATE" => self.change_date = Some(ChangeDate::new(tokenizer, level + 1)),
            "DESC" => self.generations_of_descendants = Some(tokenizer.take_line_value()),
            "FAMF" => self.name_of_family_file = Some(tokenizer.take_line_value()),
            "NOTE" => self.note = Some(Note::new(tokenizer, level + 1)),
            "ORDI" => self.ordinance_process_flag = Some(tokenizer.take_line_value()),
            "RIN" => self.automated_record_id = Some(tokenizer.take_line_value()),
            "SUBM" => self.submitter_link = Some(tokenizer.take_line_value()),
            "TEMP" => self.temple_code = Some(tokenizer.take_line_value()),
            _ => panic!(
                "{}, Unhandled SubmissionRecord tag: {}",
                tokenizer.debug(),
                tag
            ),
        };
        self.custom_data = parse_subset(tokenizer, level, handle_subset);
    }
}
