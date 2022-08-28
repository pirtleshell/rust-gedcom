#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// A copyright statement, as appropriate for the copyright laws applicable to this data.
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#COPR
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Copyright {
    pub value: Option<String>,
    /// tag: CONT
    pub continued: Option<String>,
}

