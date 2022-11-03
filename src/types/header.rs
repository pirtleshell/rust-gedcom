use crate::types::{Copyright, Corporation, Date, Note};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

use super::CustomData;

/// Header (tag: HEAD) containing GEDCOM metadata.
/// See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEADER
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Header {
    /// tag: GEDC
    pub gedcom: Option<GedcomDocument>,
    /// tag: CHAR
    pub encoding: Option<Encoding>,
    /// tag: SOUR
    pub source: Option<HeadSource>,
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
    pub fn add_custom_data(&mut self, data: CustomData) {
        self.custom_data.push(data)
    }
}

/// GedcomDocument (tag: GEDC) is a container for information about the entire document. It is
/// recommended that applications write GEDC with its required subrecord VERS as the first
/// substructure of a HEAD. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#GEDC
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct GedcomDocument {
    /// tag: VERS
    pub version: Option<String>,
    /// tag: FORM; see Gedcom 5.5.1 specification, p. 50
    pub form: Option<String>,
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

/// HeadSource (tag: SOUR) is an identifier for the product producing the gedcom data. A
/// registration process for these identifiers existed for a time, but no longer does. If an
/// existing identifier is known, it should be used. Otherwise, a URI owned by the product should
/// be used instead. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#HEAD-SOUR
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct HeadSource {
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
