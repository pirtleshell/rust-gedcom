#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// TODO Date should encompasses a number of date formats, e.g. approximated, period, phrase and range.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Date {
    pub value: Option<String>,
    pub time: Option<String>,
}

impl Date {
    /// datetime returns Date and Date.time in a single string.
    pub fn datetime(&self) -> Option<String> {
        match &self.time {
            Some(time) => {
                let mut dt = String::new();
                dt.push_str(self.value.as_ref().unwrap().as_str());
                dt.push_str(" ");
                dt.push_str(&time);
                Some(dt)
            }
            None => None,
        }
    }
}

/// ChangeDate is intended to only record the last change to a record. Some systems may want to
/// manage the change process with more detail, but it is sufficient for GEDCOM purposes to
/// indicate the last time that a record was modified.
#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ChangeDate {
    pub date: Option<Date>,
    pub note: Option<String>,
}
