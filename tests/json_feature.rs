#[cfg(test)]
mod lib;

#[cfg(test)]
#[cfg(feature = "json")]
mod json_feature_tests {
    use super::lib::util::read_relative;
    use gedcom::{parse, types::Name};
    use serde_json;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn serde_simple_gedcom_data() {
        let name = Name {
            value: Some("Gregor Johann /Mendel/".into()),
            given: Some("Gregor Johann".into()),
            surname: Some("Mendel".into()),
            prefix: None,
            surname_prefix: None,
            suffix: None,
        };

        assert_tokens(
            &name,
            &[
                Token::Struct {
                    name: "Name",
                    len: 6,
                },
                Token::Str("value"),
                Token::Some,
                Token::String("Gregor Johann /Mendel/"),
                Token::Str("given"),
                Token::Some,
                Token::String("Gregor Johann"),
                Token::Str("surname"),
                Token::Some,
                Token::String("Mendel"),
                Token::Str("prefix"),
                Token::None,
                Token::Str("surname_prefix"),
                Token::None,
                Token::Str("suffix"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serde_entire_gedcom_tree() {
        let gedcom_content: String = read_relative("./tests/fixtures/simple.ged");
        let data = parse(gedcom_content.chars());

        assert_eq!(
            serde_json::to_string_pretty(&data.families).unwrap(),
            "[
  {
    \"xref\": \"@FAMILY@\",
    \"individual1\": \"@FATHER@\",
    \"individual2\": \"@MOTHER@\",
    \"children\": [
      \"@CHILD@\"
    ],
    \"num_children\": null,
    \"events\": [
      {
        \"event\": \"Marriage\",
        \"date\": \"1 APR 1950\",
        \"place\": \"marriage place\",
        \"citations\": []
      }
    ]
  }
]"
        );

        assert_eq!(
            serde_json::to_string_pretty(&data.individuals).unwrap(),
            "[
  {
    \"xref\": \"@FATHER@\",
    \"name\": {
      \"value\": \"/Father/\",
      \"given\": null,
      \"surname\": null,
      \"prefix\": null,
      \"surname_prefix\": null,
      \"suffix\": null
    },
    \"sex\": \"Male\",
    \"families\": [
      [
        \"@FAMILY@\",
        \"Spouse\",
        null
      ]
    ],
    \"custom_data\": [],
    \"last_updated\": null,
    \"events\": [
      {
        \"event\": \"Birth\",
        \"date\": \"1 JAN 1899\",
        \"place\": \"birth place\",
        \"citations\": []
      },
      {
        \"event\": \"Death\",
        \"date\": \"31 DEC 1990\",
        \"place\": \"death place\",
        \"citations\": []
      }
    ]
  },
  {
    \"xref\": \"@MOTHER@\",
    \"name\": {
      \"value\": \"/Mother/\",
      \"given\": null,
      \"surname\": null,
      \"prefix\": null,
      \"surname_prefix\": null,
      \"suffix\": null
    },
    \"sex\": \"Female\",
    \"families\": [
      [
        \"@FAMILY@\",
        \"Spouse\",
        null
      ]
    ],
    \"custom_data\": [],
    \"last_updated\": null,
    \"events\": [
      {
        \"event\": \"Birth\",
        \"date\": \"1 JAN 1899\",
        \"place\": \"birth place\",
        \"citations\": []
      },
      {
        \"event\": \"Death\",
        \"date\": \"31 DEC 1990\",
        \"place\": \"death place\",
        \"citations\": []
      }
    ]
  },
  {
    \"xref\": \"@CHILD@\",
    \"name\": {
      \"value\": \"/Child/\",
      \"given\": null,
      \"surname\": null,
      \"prefix\": null,
      \"surname_prefix\": null,
      \"suffix\": null
    },
    \"sex\": \"Unknown\",
    \"families\": [
      [
        \"@FAMILY@\",
        \"Child\",
        null
      ]
    ],
    \"custom_data\": [],
    \"last_updated\": null,
    \"events\": [
      {
        \"event\": \"Birth\",
        \"date\": \"31 JUL 1950\",
        \"place\": \"birth place\",
        \"citations\": []
      },
      {
        \"event\": \"Death\",
        \"date\": \"29 FEB 2000\",
        \"place\": \"death place\",
        \"citations\": []
      }
    ]
  }
]"
        );

        // let json_data = serde_json::to_string_pretty(&data.individuals).unwrap();
        // panic!("{:?}", json_data);
    }
}
