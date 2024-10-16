use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct GenotypeParser;

pub fn parse_gt_code(code: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    GenotypeParser::parse(Rule::module, code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_alias() {
        parse_file("./examples/syntax/01-alias.type");
    }

    #[test]
    fn test_primitives() {
        parse_file("./examples/syntax/02-primitives.type");
    }

    #[test]
    fn test_objects() {
        parse_file("./examples/syntax/03-objects.type");
    }

    #[test]
    fn test_comments() {
        parse_file("./examples/syntax/04-comments.type");
    }

    #[test]
    fn test_optional() {
        parse_file("./examples/syntax/05-optional.type");
    }

    #[test]
    fn test_nested() {
        parse_file("./examples/syntax/06-nested.type");
    }

    #[test]
    fn test_arrays() {
        parse_file("./examples/syntax/07-arrays.type");
    }

    #[test]
    fn test_tuples() {
        parse_file("./examples/syntax/08-tuples.type");
    }

    #[test]
    fn test_modules() {
        parse_file("./examples/syntax/09-modules.type");
    }

    #[test]
    fn test_extensions() {
        parse_file("./examples/syntax/10-extensions.type");
    }

    #[test]
    fn test_literals() {
        parse_file("./examples/syntax/11-literals.type");
    }

    // #[test]
    // fn test_unions() {
    //     parse_file("./examples/syntax/12-unions.type");
    // }

    fn parse_file(file: &str) {
        let code = fs::read_to_string(file).expect("cannot read file");
        let pairs = parse_gt_code(&code);

        if let Err(err) = pairs {
            println!("{}", err);
            assert!(false, "Failed to parse file");
        }
    }
}
