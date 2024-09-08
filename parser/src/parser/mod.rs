use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct GenotypeParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use std::fs;

    #[test]
    fn test_alias() {
        parse_file("../examples/basic/alias.type");
    }

    #[test]
    fn test_primitives() {
        parse_file("../examples/basic/primitives.type");
    }

    #[test]
    fn test_struct() {
        parse_file("../examples/basic/struct.type");
    }

    #[test]
    fn test_comments() {
        parse_file("../examples/basic/comments.type");
    }

    #[test]
    fn test_optional() {
        parse_file("../examples/basic/optional.type");
    }

    fn parse_file(file: &str) {
        let file = fs::read_to_string(file).expect("cannot read file");
        let parse = GenotypeParser::parse(Rule::file, &file);

        if let Err(err) = parse {
            println!("{}", err);
            assert!(false, "Failed to parse file");
        }
    }
}
