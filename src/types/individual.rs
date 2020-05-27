use crate::types::Event;

type Xref = String;

#[derive(Debug)]
pub struct Individual {
    pub xref: Option<Xref>,
    pub name: Option<Name>,
    pub sex: Gender,
    pub events: Vec<Event>,
    pub families: Vec<FamilyLink>,
}

impl Individual {
    pub fn empty(xref: Option<Xref>) -> Individual {
        Individual {
            xref,
            name: None,
            sex: Gender::Unknown,
            events: Vec::new(),
            families: Vec::new(),
        }
    }

    pub fn add_family(&mut self, xref: Xref, tag: &str) {
        let mut do_add = true;
        for FamilyLink(family, _) in self.families.iter() {
            if family.as_str() == xref.as_str() { do_add = false; }
        }
        if do_add {
            let link_type = match tag {
                "FAMC" => FamilyLinkType::Child,
                "FAMS" => FamilyLinkType::Spouse,
                _ => panic!("Unrecognized family type tag: {}", tag),
            };
            self.families.push(FamilyLink(xref, link_type));
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

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
pub struct FamilyLink(Xref, FamilyLinkType);

#[derive(Debug)]
pub struct Name {
    pub value: Option<String>,
    pub given: Option<String>,
    pub surname: Option<String>,
}
