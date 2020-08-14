use crate::types::{Address, Submitter, SourceData};

type Xref = String;

/// A struct representing a header based on this specification

/// SOUR <`APPROVED_SYSTEM_ID`> 
/// +2 VERS <`VERSION_NUMBER`> 
/// +2 NAME <`NAME_OF_PRODUCT`> 
/// +2 CORP <`NAME_OF_BUSINESS`>
///+3 <<`ADDRESS_STRUCTURE`>>
///+2 DATA <`NAME_OF_SOURCE_DATA`>
///+3 DATE <`PUBLICATION_DATE`>
///+3 COPR <`COPYRIGHT_SOURCE_DATA`>
///+4 [CONT|CONC]<`COPYRIGHT_SOURCE_DATA`>
///+1 DEST <`RECEIVING_SYSTEM_NAME`>
///+1 DATE <`TRANSMISSION_DATE`>
///+2 TIME <`TIME_VALUE`>
///+1 SUBM @<XREF:SUBM>@
///+1 SUBN @<XREF:SUBN>@
///+1 FILE <`FILE_NAME`>
///+1 COPR <`COPYRIGHT_GEDCOM_FILE`> 
///+1 GEDC
///+2 VERS <`VERSION_NUMBER`>
///+2 FORM <`GEDCOM_FORM`>
///+1 CHAR <`CHARACTER_SET`>
///+2 VERS <`VERSION_NUMBER`>
///+1 LANG <`LANGUAGE_OF_TEXT`>
///+1 PLAC
///+2 FORM <`PLACE_HIERARCHY`>
///+1 NOTE <`GEDCOM_CONTENT_DESCRIPTION`>
/// 2 [CONC|CONT] <`GEDCOM_CONTENT_DESCRIPTION`>


// I don't know what to do about these:
//PLACE_HIERARCHY, GEDCOM_CONTENT_DESCRIPTION, COPYRIGHT_GEDCOM_FILE

#[derive(Debug)]
pub struct Header {
    pub xref: Option<Xref>,
    pub id: Option<u32>,
    pub version_number: Option<u64>,
    pub product_name: Option<String>,
    pub business_name: Option<String>,
    pub language_of_text: Option<Language>,
    pub address: Option<Address>,
    pub source_name: Option<String>,
    pub gedcom_form_path: Option<String>, // not too sure about this one, but I'm rolling with it for now
    pub date: Option<String>,
    pub source_data: Option<SourceData>,
    pub reciever_name: Option<String>,
    pub transmission_date: Option<String>,
    pub time: Option<String>,
    pub filename: Option<String>,
    pub character_set: Option<Vec<char>>, // not too sure about this one, but I'm rolling with it for now
    pub submitter: Option<Submitter>,
    pub place: Option<String>
}
#[derive(Debug)]
// NOTE: Theres a dead code warning here but I think we can figure out a way to impl these 
//We can always add more to this enum, this is just for minimum viable product's sake
enum LanguageId {
    English,
    Spanish,
    Chinese,
    Korean,
    Mandarin
}

#[derive(Debug)]
pub struct Language (
Option<LanguageId>
);

impl Header {
#[must_use]
pub fn new(xref: Option<Xref>) -> Header {
    Header{
        xref,
        id: None,
        version_number: None,
        product_name: None,
        character_set: None,
        gedcom_form_path: None, 
        business_name: None,
        address: None,
        source_name: None,
        language_of_text: None,
        date: None,
        source_data: None,
        reciever_name: None,
        transmission_date: None,
        time: None,
        filename: None,
        submitter: None,
        place: None
    }
}


#[must_use]
pub fn get_id(h: &Header) -> std::option::Option<u32> {
    return h.id;
}
#[must_use]
pub fn get_version(h: &Header) -> std::option::Option<u64> {
    return h.version_number;
}
#[must_use]
pub fn get_xref(h: &Header) -> std::option::Option<&std::string::String> {
    return h.xref.as_ref();
}
}
