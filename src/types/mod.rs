// holy wow, this data format is heteronormative af...

mod events;
pub use events::*;

type xref = String;

// top-level record types
pub struct Family {
    // this data representation understands that HUSB & WIFE are just poorly-named
    // pointers to individals
    pub person1: Option<xref>, // mapped from HUSB
    pub person2: Option<xref>, // mapped from WIFE
    pub children: Vec<xref>,
    pub num_children: Option<u8>,
}

pub struct Individual {
    pub xref: Option<xref>,
    pub name: Option<String>,
    pub sex: Option<char>,
    pub birth: Option<Event>,
    pub death: Option<Event>,
}

pub struct Media {}

pub struct Repository {}

pub struct Source {}

#[derive(Debug)]
pub struct Submitter {
    pub xref: Option<xref>,
    pub name: Option<String>,
    pub address: Option<String>,
}
