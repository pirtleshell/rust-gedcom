use crate::{
    tokenizer::{Token, Tokenizer},
    Parser,
};

/// UserDefinedData handles User Defined Data. See Gedcom 5.5 spec, p.56
///
/// ```
/// use gedcom::GedcomDocument;
///
/// let sample = "\
///     0 HEAD\n\
///     1 GEDC\n\
///     2 VERS 5.5\n\
///     0 @S1207169483@ SOUR\n\
///     1 TITL New York, U.S., New York National Guard Service Cards, 1917-1954\n\
///     0 @P10@ INDI\n\
///     1 _MILT \n\
///     2 DATE 3 Nov 1947\n\
///     2 PLAC Rochester, New York, USA\n\
///     2 SOUR @S1207169483@\n\
///     3 PAGE New York State Archives; Albany, New York; Collection: New York, New York National Guard Service Cards, 1917-1954; Series: Xxxxx; Film Number: Xx\n\
///     0 TRLR";
///
/// let mut doc = GedcomDocument::new(sample.chars());
/// let data = doc.parse_document();
///
/// let custom = &data.individuals[0].custom_data;
/// assert_eq!(custom.len(), 1);
/// assert_eq!(custom[0].as_ref().tag, "_MILT");
///
/// let cs_date = custom[0].as_ref().children[0].as_ref();
/// assert_eq!(cs_date.tag, "DATE");
/// assert_eq!(cs_date.value.as_ref().unwrap(), "3 Nov 1947");
///
/// let cs_plac = custom[0].as_ref().children[1].as_ref();
/// assert_eq!(cs_plac.tag, "PLAC");
/// assert_eq!(cs_plac.value.as_ref().unwrap(), "Rochester, New York, USA");
///
/// let cs_sour = custom[0].as_ref().children[2].as_ref();
/// assert_eq!(cs_sour.tag, "SOUR");
/// assert_eq!(cs_sour.value.as_ref().unwrap(), "@S1207169483@");
///
/// let cs_sour_page = cs_sour.children[0].as_ref();
/// assert_eq!(cs_sour_page.tag, "PAGE");
/// assert_eq!(cs_sour_page.value.as_ref().unwrap(), "New York State Archives; Albany, New York; Collection: New York, New York National Guard Service Cards, 1917-1954; Series: Xxxxx; Film Number: Xx");
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct UserDefinedDataset {
    pub tag: String,
    pub value: Option<String>,
    pub children: Vec<Box<UserDefinedDataset>>,
}

impl UserDefinedDataset {
    #[must_use]
    pub fn new(tokenizer: &mut Tokenizer, level: u8, tag: &str) -> UserDefinedDataset {
        let mut udd = UserDefinedDataset {
            tag: tag.to_string(),
            value: None,
            children: Vec::new(),
        };
        udd.parse(tokenizer, level);
        udd
    }

    pub fn add_child(&mut self, child: UserDefinedDataset) {
        self.children.push(Box::new(child));
    }
}

impl Parser for UserDefinedDataset {
    fn parse(&mut self, tokenizer: &mut Tokenizer, level: u8) {
        // skip ahead of initial tag
        tokenizer.next_token();

        let mut has_child = false;
        loop {
            if let Token::Level(current) = tokenizer.current_token {
                if current <= level {
                    break;
                }
                if current > level {
                    has_child = true;
                }
            }

            match &tokenizer.current_token {
                Token::Tag(tag) => {
                    if has_child {
                        let tag_clone = tag.clone();
                        self.add_child(UserDefinedDataset::new(tokenizer, level + 1, &tag_clone))
                    }
                }
                Token::CustomTag(tag) => {
                    if has_child {
                        let tag_clone = tag.clone();
                        self.add_child(UserDefinedDataset::new(tokenizer, level + 1, &tag_clone))
                    }
                }
                Token::LineValue(val) => {
                    self.value = Some(val.to_string());
                    tokenizer.next_token();
                }
                Token::Level(_) => tokenizer.next_token(),
                Token::EOF => break,
                _ => panic!(
                    "{}, Unhandled Token in UserDefinedDataset: {:?}",
                    tokenizer.debug(),
                    tokenizer.current_token
                ),
            }
        }
    }
}
