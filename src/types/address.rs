#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// Physical address at which a fact occurs
#[derive(Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Address {
    pub value: Option<String>,
    pub adr1: Option<String>,
    pub adr2: Option<String>,
    pub adr3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub post: Option<String>,
    pub country: Option<String>,
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Address");

        fmt_optional_value!(debug, "value", &self.value);
        fmt_optional_value!(debug, "adr1", &self.adr1);
        fmt_optional_value!(debug, "adr2", &self.adr2);
        fmt_optional_value!(debug, "adr3", &self.adr3);
        fmt_optional_value!(debug, "city", &self.city);
        fmt_optional_value!(debug, "state", &self.state);
        fmt_optional_value!(debug, "post", &self.post);
        fmt_optional_value!(debug, "country", &self.country);

        debug.finish()
    }
}
