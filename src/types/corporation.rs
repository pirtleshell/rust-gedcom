use crate::types::Address;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Corporation (tag: CORP) is the name of the business, corporation, or person that produced or
/// commissioned the product. See https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#CORP
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Corporation {
    pub value: Option<String>,
    /// tag: ADDR
    pub address: Option<Address>,
    /// tag: PHON
    pub phone: Option<String>,
    /// tag: EMAIL
    pub email: Option<String>,
    /// tag: FAX
    pub fax: Option<String>,
    /// tag: WWW
    pub website: Option<String>,
}

