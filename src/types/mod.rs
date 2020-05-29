// holy wow, this data format is heteronormative af...

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

#[derive(Debug)]
pub struct Media {}

#[derive(Debug)]
pub struct Repository {
    pub xref: Option<Xref>,
    pub name: Option<String>,
    pub address: Option<Address>,
}

#[derive(Clone, Debug)]
pub struct SourceCitation {
    pub xref: Xref,
    pub page: Option<String>,
}

#[derive(Debug)]
pub struct RepoCitation {
    pub xref: Xref,
    pub call_number: Option<String>,
}

#[derive(Debug)]
pub struct Submitter {
    pub xref: Option<Xref>,
    pub name: Option<String>,
    pub address: Option<Address>,
    pub phone: Option<String>,
}

impl Submitter {
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
