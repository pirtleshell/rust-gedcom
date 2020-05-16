#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(gedcom::do_a_test(), "did a test");
    }

    #[test]
    fn test_internal_mod() {
      assert_eq!(gedcom::i_return_hello(), "hello");
    }
}
