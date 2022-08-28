use crate::types::{Translation, Source};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Note (tag:NOTE) is a note_structure, containing additional information provided by the
/// submitter for understanding the enclosing data.
///
/// When a substructure of HEAD, it should describe the contents of the document in terms of
/// “ancestors or descendants of” so that the person receiving the data knows what genealogical
/// information the document contains.
///
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#NOTE
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Note {
    pub value: Option<String>,
    /// tag: MIME, indicates the media type of the payload of the superstructure, as defined by BCP
    /// 13. As of version 7.0, only 2 media types are supported by this structure: text/plain and
    /// text/html
    pub mime: Option<String>,
    /// tag: TRAN, a type of TRAN for unstructured human-readable text, such as is found in NOTE
    /// and SNOTE payloads.
    pub translation: Option<Translation>,
    /// tag: SOUR, a citation indicating that the pointed-to source record supports the claims made
    /// in the superstructure. See
    /// https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#SOURCE_CITATION
    pub citation: Option<Source>,
    /// tag: LANG, The primary human language of the superstructure. The primary language in which
    /// the Text-typed payloads of the superstructure and its substructures appear. See
    /// https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#LANG
    pub language: Option<String>,
}
