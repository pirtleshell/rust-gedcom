# rust-gedcom

<a href="https://crates.io/crates/gedcom">
    <img style="display: inline!important" src="https://img.shields.io/crates/v/gedcom.svg"></img>
</a>
<a href="https://docs.rs/gedcom">
    <img style="display: inline!important" src="https://docs.rs/gedcom/badge.svg"></img>
</a>

> A gedcom parser written in rust 🦀

## About this project

GEDCOM is a file format for sharing genealogical information like family trees.

I wanted experience playing with parsers and representing tree structures in Rust, and noticed a parser for Rust did not exist. And thus, this project was born! A fun experiment to practice my Rust abilities.

It hopes to be ~~fully~~ mostly compliant with the [Gedcom 5.5.1 specification](https://edge.fscdn.org/assets/img/documents/ged551-5bac5e57fe88dd37df0e153d9c515335.pdf).

I have found this [5.5.2 specification](https://jfcardinal.github.io/GEDCOM-5.5.2/gedcom-5.5.2.html) useful in its assessment of which tags are worth supporting or not.

## Usage

This crate comes in two parts. The first is a binary called `parse_gedcom`, mostly used for my testing & development. It prints the `GedcomData` object and some stats about the gedcom file passed into it:
```bash
parse_gedcom ./tests/fixtures/sample.ged

# outputs tree data here w/ stats
# ----------------------
# | Gedcom Data Stats: |
# ----------------------
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

There are still parts of the specification not yet implemented and the project is subject to change. The way I have been developing is to take a gedcom file, attempt to parse it and act on whatever errors or omissions occur. In it's current state, it is capable of parsing the [sample.ged](tests/fixtures/sample.ged) in its entirety.

Here are some notes about parsed data & tags. Page references are to the [Gedcom 5.5.1 specification](https://edge.fscdn.org/assets/img/documents/ged551-5bac5e57fe88dd37df0e153d9c515335.pdf).

### Top-level tags

* `SUBMISSION_RECORD` - p.28 - No attempt at handling this is made.
* `NOTE_RECORD` - p.27 - Notes (`NOTE`) are also unhandled. (except in header)

Tags for families (`FAM`), individuals (`IND`), repositories (`REPO`), sources (`SOUR`), and submitters (`SUBM`) are handled. Many of the most common sub-tags for these are handled though some may not yet be parsed. Mileage may vary.

## License

© 2021, [Robert Pirtle](https://robert.pirtle.xyz/). licensed under [MIT](license.md).
