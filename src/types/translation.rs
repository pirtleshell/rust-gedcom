#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Translation (tag:TRAN) is a type of TRAN for unstructured human-readable text, such as
/// is found in NOTE and SNOTE payloads. Each NOTE-TRAN must have either a LANG substructure or a
/// MIME substructure or both. If either is missing, it is assumed to have the same value as the
/// superstructure. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#NOTE-TRAN
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Translation {
    pub value: Option<String>,
    /// tag:MIME
    pub mime: Option<String>,
    /// tag:LANG
    pub language: Option<String>,
}
