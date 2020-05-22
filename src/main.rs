use std::path::PathBuf;
mod tokenizer;
use tokenizer::Tokenizer;

fn read_relative(path: &str) -> String {
    let path_buf: PathBuf = PathBuf::from(path);
    let absolute_path: PathBuf = std::fs::canonicalize(path_buf).unwrap();
    println!("{}", absolute_path.display());
    std::fs::read_to_string(absolute_path).unwrap()
}

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    // println!("{}", current_dir.display());
    // println!();

    let contents = read_relative("./tests/fixtures/simple.ged");
    // println!("{}", contents);
    let mut tokenizer = Tokenizer::new(contents.chars());

    while !tokenizer.done() {
        tokenizer.next_token();
        println!("{:02}: {:?}", tokenizer.line, tokenizer.current_token);
    }
}
