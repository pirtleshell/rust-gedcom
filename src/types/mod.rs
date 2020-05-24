// holy wow, this data format is heteronormative af...

mod event;
pub use event::*;

type Xref = String;

// top-level record types
mod individual;
pub use individual::*;

mod family;
pub use family::*;

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
