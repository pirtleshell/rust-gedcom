use crate::types::Address;

type Xref = String;

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
