//! Structs and datatypes used to represent facts and data in a Gedcom file

// holy wow, this data format is heteronormative af...

#![allow(missing_docs)]

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

pub mod event;
pub use event::{EventDetail, Event};

pub mod date;
pub use date::{ChangeDate, Date};

mod place;
pub use place::*;

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

mod submission;
pub use submission::*;

mod submitter;
pub use submitter::*;

mod source;
pub use source::*;

mod note;
pub use note::*;

mod translation;
pub use translation::*;

mod repository;
pub use repository::*;

mod corporation;
pub use corporation::*;

mod multimedia;
pub use multimedia::*;

mod custom;
pub use custom::*;
