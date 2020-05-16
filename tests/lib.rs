#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn read_relative(path: &str) -> String {
        let path_buf: PathBuf = PathBuf::from(path);
        let absolute_path: PathBuf = std::fs::canonicalize(path_buf).unwrap();
        std::fs::read_to_string(absolute_path).unwrap()
    }

    #[test]
    fn it_works() {
        assert_eq!(gedcom::do_a_test(), "did a test");
    }

    #[test]
    fn test_internal_mod() {
      assert_eq!(gedcom::i_return_hello(), "hello");
    }

    #[test]
    fn opening_example() {
        let simple_ged: String = read_relative("./tests/fixtures/simple.ged");
        assert!(simple_ged.len() > 0);
    }
}
