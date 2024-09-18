use parser::parse_code;
use std::fs;

mod parser;
mod tree;

fn main() {
    let code = fs::read_to_string("../examples/syntax/01-alias.type").expect("cannot read file");
    let pairs = parse_code(&code);

    println!("----------- {:?}", pairs);
}
