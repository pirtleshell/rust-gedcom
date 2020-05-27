use crate::types::{
    Family,
    Individual,
    Media,
    Repository,
    Source,
    Submitter,
};

#[derive(Debug)]
pub struct GedcomData {
    // header:
    pub submitters: Vec<Submitter>,
    pub individuals: Vec<Individual>,
    pub families: Vec<Family>,
    pub repositories: Vec<Repository>,
    pub sources: Vec<Source>,
    pub multimedia: Vec<Media>,
}

// should maybe store these by xref if available?
impl GedcomData {
    pub fn new() -> GedcomData {
        GedcomData {
            submitters: Vec::new(),
            individuals: Vec::new(),
            families: Vec::new(),
            repositories: Vec::new(),
            sources: Vec::new(),
            multimedia: Vec::new(),
        }
    }

    pub fn add_individual(&mut self, individual: Individual) {
        self.individuals.push(individual);
    }

    pub fn add_family(&mut self, family: Family) {
        self.families.push(family);
    }

    pub fn add_submitter(&mut self, submitter: Submitter) {
        self.submitters.push(submitter);
    }
}
