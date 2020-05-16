mod grammar;

pub fn do_a_test() -> &'static str {
    "did a test"
}

pub fn i_return_hello() -> &'static str {
    grammar::test()
}
