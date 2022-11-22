#[cfg(test)]
mod tests {
    use gedcom::GedcomRecord;

    #[test]
    fn parses_basic_multimedia_record() {
        let sample = "\
            0 HEAD\n\
            1 CHAR UTF-8\n\
            1 SOUR Ancestry.com Family Trees\n\
            2 VERS (2010.3)\n\
            2 NAME Ancestry.com Family Trees\n\
            2 CORP Ancestry.com\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            2 FORM LINEAGE-LINKED\n\
            0 OBJE\n\
            1 FILE http://trees.ancestry.com/rd?f=image&guid=Xxxxxxxx-Xxxx-Xxxx-Xxxx-Xxxxxxxxxxxx&tid=Xxxxxxxx&pid=1\n\
            1 FORM jpg\n\
            1 TITL In Prague\n\
            0 TRLR";

        let mut record = GedcomRecord::new(sample.chars());
        let data = record.parse_record();
        assert_eq!(data.multimedia.len(), 1);

        let obje = &data.multimedia[0];
        assert_eq!(obje.title.as_ref().unwrap(), "In Prague");

        let form = obje.form.as_ref().unwrap();
        assert_eq!(form.value.as_ref().unwrap(), "jpg");

        let file = obje.file.as_ref().unwrap();
        assert_eq!(
            file.value.as_ref().unwrap(),
            "http://trees.ancestry.com/rd?f=image&guid=Xxxxxxxx-Xxxx-Xxxx-Xxxx-Xxxxxxxxxxxx&tid=Xxxxxxxx&pid=1"
        );
    }

    #[test]
    fn parses_spec_structure() {
        let sample = "\
            0 HEAD\n\
            1 GEDC\n\
            2 VERS 5.5\n\
            2 FORM LINEAGE-LINKED\n\
            0 @MEDIA1@ OBJE\n\
            1 FILE /home/user/media/file_name.bmp\n\
            2 FORM bmp\n\
            3 TYPE photo
            2 TITL A Bitmap\n\
            1 REFN 000\n\
            2 TYPE User Reference Type\n\
            1 RIN Automated Id\n\
            1 NOTE A note\n\
            2 CONT Note continued here. The word TE\n\
            2 CONC ST should not be broken!\n\
            1 SOUR @SOUR1@\n\
            2 PAGE 42
            2 _CUSTOM Custom data\n\
            1 CHAN 
            2 DATE 1 APR 1998
            3 TIME 12:34:56.789
            2 NOTE A note
            3 CONT Note continued here. The word TE
            3 CONC ST should not be broken!
            0 TRLR";

        let mut parser = GedcomRecord::new(sample.chars());
        let data = parser.parse_record();
        assert_eq!(data.multimedia.len(), 1);

        let obje = &data.multimedia[0];
        assert_eq!(obje.xref.as_ref().unwrap(), "@MEDIA1@");

        let file = obje.file.as_ref().unwrap();
        assert_eq!(
            file.value.as_ref().unwrap(),
            "/home/user/media/file_name.bmp"
        );

        assert_eq!(file.title.as_ref().unwrap(), "A Bitmap");

        let form = file.form.as_ref().unwrap();
        assert_eq!(form.value.as_ref().unwrap(), "bmp");
        assert_eq!(form.source_media_type.as_ref().unwrap(), "photo");

        let user_ref = obje.user_reference_number.as_ref().unwrap();
        assert_eq!(user_ref.value.as_ref().unwrap(), "000");
        assert_eq!(
            user_ref.user_reference_type.as_ref().unwrap(),
            "User Reference Type"
        );

        assert_eq!(obje.automated_record_id.as_ref().unwrap(), "Automated Id");

        let note = obje.note_structure.as_ref().unwrap();
        assert_eq!(
            note.value.as_ref().unwrap(),
            "A note\nNote continued here. The word TEST should not be broken!"
        );

        let sour = obje.source_citation.as_ref().unwrap();
        assert_eq!(sour.xref, "@SOUR1@");
        assert_eq!(sour.page.as_ref().unwrap(), "42");
        assert_eq!(sour.custom_data.len(), 1);
        assert_eq!(sour.custom_data[0].value, "Custom data");

        let chan = obje.change_date.as_ref().unwrap();
        let date = chan.date.as_ref().unwrap();
        assert_eq!(date.value.as_ref().unwrap(), "1 APR 1998");
        assert_eq!(date.time.as_ref().unwrap(), "12:34:56.789");

        let chan_note = chan.note.as_ref().unwrap();
        assert_eq!(chan_note.value.as_ref().unwrap(), "A note\nNote continued here. The word TEST should not be broken!");
    }
}
