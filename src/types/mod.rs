//! Structs and datatypes used to represent facts and data in a Gedcom file

// holy wow, this data format is heteronormative af...

#![allow(missing_docs)]

mod event;
pub use event::*;
mod address;
pub use address::*;

type Xref = String;

// top-level record types
mod header;
pub use header::*;

mod individual;
pub use individual::*;

mod family;
pub use family::*;

mod submitter;
pub use submitter::*;

mod source;
pub use source::*;

// TODO
/// Multimedia item
#[derive(Debug)]
pub struct Media {}

/// Data repository, the `REPO` tag
#[derive(Debug)]
pub struct Repository {
    /// Optional reference to link to this repo
    pub xref: Option<Xref>,
    /// Name of the repository
    pub name: Option<String>,
    /// Physical address of the data repository
    pub address: Option<Address>,
}

/// Citation linking a genealogy fact to a data `Source`
#[derive(Clone, Debug)]
pub struct SourceCitation {
    /// Reference to the `Source`
    pub xref: Xref,
    /// Page number of source
    pub page: Option<String>,
}

/// Citation linking a `Source` to a data `Repository`
#[derive(Debug)]
pub struct RepoCitation {
    /// Reference to the `Repository`
    pub xref: Xref,
    /// Call number to find the source at this repository
    pub call_number: Option<String>,
}

#[derive(Debug)]
pub struct CustomData {
    pub tag: String,
    pub value: String,
}
