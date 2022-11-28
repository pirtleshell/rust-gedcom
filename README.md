# rust-gedcom

<!-- <a href="https://crates.io/crates/gedcom"> -->
<!--     <img style="display: inline!important" src="https://img.shields.io/crates/v/gedcom.svg"></img> -->
<!-- </a> -->
<!-- <a href="https://docs.rs/gedcom"> -->
<!--     <img style="display: inline!important" src="https://docs.rs/gedcom/badge.svg"></img> -->
<!-- </a> -->

> A gedcom parser written in rust 🦀

## About this project

GEDCOM is a file format for sharing genealogical information like family trees.

`rust-gedcom` hopes to be ~~fully~~ mostly compliant with the [Gedcom 5.5.1 specification](https://edge.fscdn.org/assets/img/documents/ged551-5bac5e57fe88dd37df0e153d9c515335.pdf).

Later specifications, such as [5.5.2](https://jfcardinal.github.io/GEDCOM-5.5.2/gedcom-5.5.2.html) and [7.0.11](https://gedcom.io/specifications/FamilySearchGEDCOMv7.html#purpose-and-content-of-the-familysearch-gedcom-specification), are useful in assessing which tags are worth supporting or not.

## Usage

This crate comes in two parts. The first is a binary called `parse_gedcom`, mostly used for testing & development. It prints the `GedcomData` object and some stats about the GEDCOM file passed into it:
```bash
parse_gedcom ./tests/fixtures/sample.ged

# outputs tree data here w/ stats
# ----------------------
# | Gedcom Data Stats: |
# ----------------------
#   submissions: 0
#   submitters: 1
#   individuals: 3
#   families: 2
#   repositories: 1
#   sources: 1
#   multimedia: 0
# ----------------------
```

The second is a library containing the parser.

## JSON Serializing/Deserializing with `serde`
This crate has an optional feature called `json` that implements `Serialize` & `Deserialize` for the gedcom data structure. This allows you to easily integrate with the web.

For more info about serde, [check them out](https://serde.rs/)!

The feature is not enabled by default. There are zero dependencies if just using the gedcom parsing functionality.

Use the json feature with any version >=0.2.1 by adding the following to your Cargo.toml:
```toml
gedcom = { version = "<version>", features = ["json"] }
```

## 🚧 Progress 🚧

There are still parts of the specification not yet implemented, and the project is subject to change. The way development has been happening is by taking a GEDCOM file, attempting to parse it and acting on whatever errors or omissions occur. In its current state, it is capable of parsing the [sample.ged](tests/fixtures/sample.ged) in its entirety.

Here are some notes about parsed data & tags. Page references are to the [Gedcom 5.5.1 specification](https://edge.fscdn.org/assets/img/documents/ged551-5bac5e57fe88dd37df0e153d9c515335.pdf).

### Top-level tags

Tags for families (`FAM`), individuals (`IND`), repositories (`REPO`), sources (`SOUR`), and submitters (`SUBM`) are handled. Many of the most common sub-tags for these are handled though some may not yet be parsed. Mileage may vary.

## License

Licensed under [MIT](license.md).
