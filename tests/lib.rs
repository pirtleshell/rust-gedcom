#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn read_relative(path: &str) -> String {
        let path_buf: PathBuf = PathBuf::from(path);
        let absolute_path: PathBuf = std::fs::canonicalize(path_buf).unwrap();
        std::fs::read_to_string(absolute_path).unwrap()
    }

    #[test]
    fn opening_example() {
        let simple_ged: String = read_relative("./tests/fixtures/simple.ged");
        assert!(simple_ged.len() > 0);
    }
}
