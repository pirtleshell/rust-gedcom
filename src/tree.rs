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
    submitters: Vec<Submitter>,
    individuals: Vec<Individual>,
    families: Vec<Family>,
    repositories: Vec<Repository>,
    sources: Vec<Source>,
    multimedia: Vec<Media>,
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
