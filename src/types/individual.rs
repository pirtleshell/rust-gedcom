use crate::types::Event;

type Xref = String;

#[derive(Debug)]
pub struct Individual {
    pub xref: Option<Xref>,
    pub name: Option<String>,
    pub sex: Gender,
    pub birth: Option<Event>,
    pub death: Option<Event>,
    pub families: Vec<FamilyLink>,
}
impl Individual {
    pub fn empty(xref: Option<Xref>) -> Individual {
        Individual {
            xref,
            name: None,
            sex: Gender::Unknown,
            birth: None,
            death: None,
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
