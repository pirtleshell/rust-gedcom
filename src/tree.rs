use crate::types::{Family, Header, Individual, Media, Repository, Source, Submitter};
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
/// The data structure representing all the data within a gedcom file
pub struct GedcomData {
    /// Header containing file metadata
    pub header: Header,
    /// List of submitters of the facts
    pub submitters: Vec<Submitter>,
    /// Individuals within the family tree
    pub individuals: Vec<Individual>,
    /// The family units of the tree, representing relationships between individuals
    pub families: Vec<Family>,
    /// A data repository where `sources` are held
    pub repositories: Vec<Repository>,
    /// Sources of facts. _ie._ book, document, census, etc.
    pub sources: Vec<Source>,
    /// A multimedia asset linked to a fact
    pub multimedia: Vec<Media>,
}

// should maybe store these by xref if available?
impl GedcomData {
    /// Adds a `Family` (a relationship between individuals) to the tree
    pub fn add_family(&mut self, family: Family) {
        self.families.push(family);
    }

    /// Adds an `Individual` to the tree
    pub fn add_individual(&mut self, individual: Individual) {
        self.individuals.push(individual);
    }

    /// Adds a data `Repository` to the tree
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Adds a `Source` to the tree
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    /// Adds a `Submitter` to the tree
    pub fn add_submitter(&mut self, submitter: Submitter) {
        self.submitters.push(submitter);
    }

    /// Outputs a summary of data contained in the tree to stdout
    pub fn stats(&self) {
        println!("----------------------");
        println!("| Gedcom Data Stats: |");
        println!("----------------------");
        println!("  submitters: {}", self.submitters.len());
        println!("  individuals: {}", self.individuals.len());
        println!("  families: {}", self.families.len());
        println!("  repositories: {}", self.repositories.len());
        println!("  sources: {}", self.sources.len());
        println!("  multimedia: {}", self.multimedia.len());
        println!("----------------------");
    }
}
