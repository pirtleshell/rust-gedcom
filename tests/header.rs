#[cfg(test)]
mod tests {
    use gedcom::GedcomRecord;

    #[test]
    fn parse_head_gedc() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            2 FORM LINEAGE-LINKED\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let head_gedc = data.header.unwrap().gedcom.unwrap();
        assert_eq!(head_gedc.version.unwrap(), "5.5");
        assert_eq!(head_gedc.form.unwrap(), "LINEAGE-LINKED");
    }

    #[test]
    fn parse_head_sour() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 SOUR SOURCE_NAME\n\
            2 VERS Version number of source-program\n\
            2 NAME Name of source-program\n\
            2 CORP Corporation name\n\
            3 ADDR 2 Harrison Street\n\
            4 CONT 7th Floor\n\
            4 CONT Suite 175\n\
            4 ADR1 2 Harrison Street\n\
            4 ADR2 7th Floor\n\
            4 ADR3 Suite 175\n\
            4 CITY San Francisco\n\
            4 STAE California\n\
            4 POST 94105\n\
            4 CTRY USA\n\
            3 PHON Corporation phone number\n\
            2 DATA Name of source data\n\
            3 DATE 1 JAN 1998\n\
            3 COPR Copyright of source data\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let sour = data.header.unwrap().source.unwrap();
        assert_eq!(sour.value.unwrap(), "SOURCE_NAME");

        let vers = sour.version.unwrap();
        assert_eq!(vers, "Version number of source-program");

        let name = sour.name.unwrap();
        assert_eq!(name, "Name of source-program");

        let corp = sour.corporation.unwrap();
        assert_eq!(corp.value.unwrap(), "Corporation name");

        let corp_addr = corp.address.unwrap();
        assert_eq!(
            corp_addr.value.unwrap(),
            "2 Harrison Street\n7th Floor\nSuite 175"
        );
        assert_eq!(corp_addr.adr1.unwrap(), "2 Harrison Street");
        assert_eq!(corp_addr.adr2.unwrap(), "7th Floor");
        assert_eq!(corp_addr.adr3.unwrap(), "Suite 175");
        assert_eq!(corp_addr.city.unwrap(), "San Francisco");
        assert_eq!(corp_addr.state.unwrap(), "California");
        assert_eq!(corp_addr.post.unwrap(), "94105");
        assert_eq!(corp_addr.country.unwrap(), "USA");

        let corp_phon = corp.phone.unwrap();
        assert_eq!(corp_phon, "Corporation phone number");

        let sour_data = sour.data.unwrap();
        assert_eq!(sour_data.value.unwrap(), "Name of source data");
        assert_eq!(sour_data.date.unwrap().value.unwrap(), "1 JAN 1998");
        assert_eq!(
            sour_data.copyright.unwrap().value.unwrap(),
            "Copyright of source data"
        );
    }

    #[test]
    fn parse_head_dest() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 DEST Destination of transmission\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        assert_eq!(
            data.header.unwrap().destination.unwrap(),
            "Destination of transmission"
        );
    }

    #[test]
    fn parse_head_date() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 DATE 1 JAN 1998\n\
            2 TIME 13:57:24.80\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_date = data.header.unwrap().date.unwrap();
        assert_eq!(h_date.value.unwrap(), "1 JAN 1998");
        assert_eq!(h_date.time.unwrap(), "13:57:24.80");
    }

    #[test]
    fn parse_head_subm() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 SUBM @SUBMITTER@\n\
            1 SUBN @SUBMISSION@\n\
            1 FILE ALLGED.GED\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_subm = data.header.unwrap().submitter_tag.unwrap();
        assert_eq!(h_subm.as_str(), "@SUBMITTER@");
    }

    #[test]
    fn parse_head_subn() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 SUBM @SUBMITTER@\n\
            1 SUBN @SUBMISSION@\n\
            1 FILE ALLGED.GED\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_subn = data.header.unwrap().submission_tag.unwrap();
        assert_eq!(h_subn.as_str(), "@SUBMISSION@");
    }

    #[test]
    fn parse_head_file() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 SUBM @SUBMITTER@\n\
            1 SUBN @SUBMISSION@\n\
            1 FILE ALLGED.GED\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_file = data.header.unwrap().filename.unwrap();
        assert_eq!(h_file.as_str(), "ALLGED.GED");
    }

    #[test]
    fn parse_head_copr() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 COPR (C) 1997-2000 by H. Eichmann.\n\
            2 CONT You can use and distribute this file freely as long as you do not charge for it.\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_copr = data.header.unwrap().copyright.unwrap();
        assert_eq!(h_copr.value.unwrap(), "(C) 1997-2000 by H. Eichmann.");
        assert_eq!(
            h_copr.continued.unwrap(),
            "You can use and distribute this file freely as long as you do not charge for it."
        );
    }

    #[test]
    fn parse_head_char() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 CHAR ASCII\n\
            2 VERS Version number of ASCII (whatever it means)\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_char = data.header.unwrap().encoding.unwrap();
        assert_eq!(h_char.value.unwrap(), "ASCII");
        assert_eq!(
            h_char.version.unwrap(),
            "Version number of ASCII (whatever it means)"
        );
    }

    #[test]
    fn parse_head_lang() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 LANG language
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_lang = data.header.unwrap().language.unwrap();
        assert_eq!(h_lang.as_str(), "language");
    }

    #[test]
    fn parse_head_plac() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 PLAC\n\
            2 FORM City, County, State, Country\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_plac = data.header.unwrap().place.unwrap();
        assert_eq!(h_plac.form[0], "City");
        assert_eq!(h_plac.form[1], "County");
        assert_eq!(h_plac.form[2], "State");
        assert_eq!(h_plac.form[3], "Country");
    }

    #[test]
    fn parse_head_note() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            1 NOTE A general note about this file:\n\
            2 CONT It demonstrates most of the data which can be submitted using GEDCOM5.5. It shows the relatives of PERSON1:\n\
            2 CONT His 2 wifes (PERSON2, PERSON8), his parents (father: PERSON5, mother not given),\n\
            2 CONT adoptive parents (mother: PERSON6, father not given) and his 3 children (PERSON3, PERSON4 and PERSON7).\n\
            2 CONT In PERSON1, FAMILY1, SUBMITTER, SUBMISSION and SOURCE1 as many datafields as possible are used.\n\
            2 CONT All other individuals/families contain no data. Note, that many data tags can appear more than once\n\
            2 CONT (in this transmission this is demonstrated with tags: NAME, OCCU, PLACE and NOTE. Seek the word 'another'.\n\
            2 CONT The data transmitted here do not make sence. Just the HEAD.DATE tag contains the date of the creation\n\
            2 CONT of this file and will change in future Versions!\n\
            2 CONT This file is created by H. Eichmann: h.eichmann@@gmx.de. Feel free to copy and use it for any\n\
            2 CONT non-commercial purpose. For the creation the GEDCOM standard Release 5.5 (2 JAN 1996) has been used.\n\
            2 CONT Copyright: gedcom@@gedcom.org\n\
            2 CONT Download it (the GEDCOM 5.5 specs) from: ftp.gedcom.com/pub/genealogy/gedcom.\n\
            2 CONT Some Specials: This line is very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very long but not too long (255 caharcters is the limit).\n\
            2 CONT This @@ (commercial at) character may only appear ONCE!\n\
            2 CONT Note continued here. The word TE\n\
            2 CONC ST should not be broken!\n\
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();

        let h_note = data.header.unwrap().note.unwrap();
        assert_eq!(h_note.value.unwrap().chars().count(), 1440);
    }
}
