//! Structs and datatypes used to represent facts and data in a Gedcom file

// holy wow, this data format is heteronormative af...

#![allow(missing_docs)]

mod event;
pub use event::*;
mod address;
pub use address::*;

type Xref = String;

// top-level record types
mod individual;
pub use individual::*;

mod family;
pub use family::*;

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

/// Submitter of the data, ie. who reported the genealogy fact
#[derive(Debug)]
pub struct Submitter {
    /// Optional reference to link to this submitter
    pub xref: Option<Xref>,
    /// Name of the submitter
    pub name: Option<String>,
    /// Physical address of the submitter
    pub address: Option<Address>,
    /// Phone number of the submitter
    pub phone: Option<String>,
}

impl Submitter {
    /// Shorthand for creating a `Submitter` from its `xref`
    #[must_use]
    pub fn new(xref: Option<Xref>) -> Submitter {
        Submitter {
            xref,
            name: None,
            address: None,
            phone: None,
        }
    }
}
