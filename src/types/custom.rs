#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct CustomData {
    pub tag: String,
    pub value: String,
}
