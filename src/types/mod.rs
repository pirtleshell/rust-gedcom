// holy wow, this data format is heteronormative af...

mod event;
pub use event::*;

type Xref = String;

// top-level record types
mod individual;
pub use individual::*;

#[derive(Debug)]
pub struct Family {
    // this data representation understands that HUSB & WIFE are just poorly-named
    // pointers to individals. no gender "validating" is done on parse.
    pub person1: Option<Xref>, // mapped from HUSB
    pub person2: Option<Xref>, // mapped from WIFE
    pub children: Vec<Xref>,
    pub num_children: Option<u8>,
}

#[derive(Debug)]
pub struct Media {}

#[derive(Debug)]
pub struct Repository {}

#[derive(Debug)]
pub struct Source {}

#[derive(Debug)]
pub struct Submitter {
    pub xref: Option<Xref>,
    pub name: Option<String>,
    pub address: Option<String>,
}
