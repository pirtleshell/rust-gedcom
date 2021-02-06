use crate::types::{Event, Individual};

type Xref = String;

/// Data for an entire collection of people and families.
#[derive(Debug)]
pub struct FamilyTree {
    pub individuals: Vec<Individual>,
}

impl FamilyTree {
    pub fn events(&self) {}
}
