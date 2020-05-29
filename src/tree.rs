use crate::types::{
    Family,
    Individual,
    Media,
    Repository,
    Source,
    Submitter,
};

#[derive(Debug, Default)]
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
    pub fn add_family(&mut self, family: Family) {
        self.families.push(family);
    }

    pub fn add_individual(&mut self, individual: Individual) {
        self.individuals.push(individual);
    }

    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    pub fn add_submitter(&mut self, submitter: Submitter) {
        self.submitters.push(submitter);
    }

    pub fn stats(&self) {
        println!("----------------------");
        println!("| Gedcom Data Stats: |");
        println!("----------------------");
        println!("  submitters: {}", self.submitters.len());
        println!("  individuals: {}", self.individuals.len());
        println!("  families: {}", self.families.len());
        println!("  sources: {}", self.sources.len());
        println!("  multimedia: {}", self.multimedia.len());
        println!("----------------------");
    }
}
