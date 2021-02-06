use crate::types::{event::HasEvents, CustomData, Event};

type Xref = String;

/// A Person within the family tree
#[derive(Debug)]
pub struct Individual {
    pub xref: Option<Xref>,
    pub name: Option<Name>,
    pub sex: Gender,
    pub families: Vec<FamilyLink>,
    pub custom_data: Vec<CustomData>,
    pub last_updated: Option<String>,
    events: Vec<Event>,
}

impl Individual {
    #[must_use]
    pub fn new(xref: Option<Xref>) -> Individual {
        Individual {
            xref,
            name: None,
            sex: Gender::Unknown,
            events: Vec::new(),
            families: Vec::new(),
            custom_data: Vec::new(),
            last_updated: None,
        }
    }

    pub fn add_family(&mut self, link: FamilyLink) {
        let mut do_add = true;
        let xref = &link.0;
        for FamilyLink(family, _, _) in &self.families {
            if family.as_str() == xref.as_str() {
                do_add = false;
            }
        }
        if do_add {
            self.families.push(link);
        }
    }

    pub fn add_custom_data(&mut self, data: CustomData) {
        self.custom_data.push(data)
    }
}

impl HasEvents for Individual {
    fn add_event(&mut self, event: Event) -> () {
        self.events.push(event);
    }
    fn events(&self) -> Vec<Event> {
        self.events.clone()
    }
}

/// Gender of an `Individual`
#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    // come at me LDS, i support "N" as a gender value
    Nonbinary,
    Unknown,
}

#[derive(Debug)]
enum FamilyLinkType {
    Spouse,
    Child,
}

#[derive(Debug)]
enum Pedigree {
    Adopted,
    Birth,
    Foster,
    Sealing,
}

#[derive(Debug)]
pub struct FamilyLink(Xref, FamilyLinkType, Option<Pedigree>);

impl FamilyLink {
    #[must_use]
    pub fn new(xref: Xref, tag: &str) -> FamilyLink {
        let link_type = match tag {
            "FAMC" => FamilyLinkType::Child,
            "FAMS" => FamilyLinkType::Spouse,
            _ => panic!("Unrecognized family type tag: {}", tag),
        };
        FamilyLink(xref, link_type, None)
    }

    pub fn set_pedigree(&mut self, pedigree_text: &str) {
        self.2 = match pedigree_text.to_lowercase().as_str() {
            "adopted" => Some(Pedigree::Adopted),
            "birth" => Some(Pedigree::Birth),
            "foster" => Some(Pedigree::Foster),
            "sealing" => Some(Pedigree::Sealing),
            _ => panic!("Unrecognized family link pedigree: {}", pedigree_text),
        };
    }
}

#[derive(Debug, Default)]
pub struct Name {
    pub value: Option<String>,
    pub given: Option<String>,
    pub surname: Option<String>,
    pub prefix: Option<String>,
    pub surname_prefix: Option<String>,
    pub suffix: Option<String>,
}
