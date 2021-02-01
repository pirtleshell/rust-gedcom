use crate::types::Source;

#[derive(Debug, Default)]
/// Header containing GEDCOM metadata
pub struct Header {
    pub encoding: Option<String>,
    pub copyright: Option<String>,
    pub corporation: Option<String>,
    pub date: Option<String>,
    pub destinations: Vec<String>,
    pub gedcom_version: Option<String>,
    pub language: Option<String>,
    pub filename: Option<String>,
    pub note: Option<String>,
    pub sources: Vec<Source>,
    pub submitter_tag: Option<String>,
    pub submission_tag: Option<String>,
}

impl Header {
    pub fn add_destination(&mut self, destination: String) {
        self.destinations.push(destination);
    }

    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }
}

// pub struct HeaderSource {
//     version: Option<String>,
//     name: Option<String>,
//     coroporation:
// }
