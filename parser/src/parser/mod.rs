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
        parse_file("../examples/02-syntax/01-alias.type");
    }

    #[test]
    fn test_primitives() {
        parse_file("../examples/02-syntax/02-primitives.type");
    }

    #[test]
    fn test_objects() {
        parse_file("../examples/02-syntax/03-objects.type");
    }

    #[test]
    fn test_comments() {
        parse_file("../examples/02-syntax/04-comments.type");
    }

    #[test]
    fn test_optional() {
        parse_file("../examples/02-syntax/05-optional.type");
    }

    #[test]
    fn test_nested() {
        parse_file("../examples/02-syntax/06-nested.type");
    }

    #[test]
    fn test_arrays() {
        parse_file("../examples/02-syntax/07-arrays.type");
    }

    #[test]
    fn test_tuples() {
        parse_file("../examples/02-syntax/08-tuples.type");
    }

    #[test]
    fn test_modules() {
        parse_file("../examples/02-syntax/09-modules.type");
    }

    #[test]
    fn test_extensions() {
        parse_file("../examples/02-syntax/10-extensions.type");
    }

    #[test]
    fn test_literals() {
        parse_file("../examples/02-syntax/11-literals.type");
    }

    #[test]
    fn test_unions() {
        parse_file("../examples/02-syntax/12-unions.type");
    }

    #[test]
    fn test_attributes() {
        parse_file("../examples/02-syntax/13-attributes.type");
    }

    #[test]
    fn test_records() {
        parse_file("../examples/02-syntax/14-records.type");
    }

    #[test]
    fn test_any() {
        parse_file("../examples/02-syntax/15-any.type");
    }

    #[test]
    fn test_branded() {
        parse_file("../examples/02-syntax/16-branded.type");
    }

    #[test]
    fn test_number_sizes() {
        parse_file("../examples/02-syntax/17-number_sizes.type");
    }

    #[test]
    fn test_empty() {
        parse_code("");
    }

    #[test]
    fn test_name() {
        parse_code(r#"HelloV1 = "hello-1""#);
    }

    fn parse_file(file: &str) {
        let code = fs::read_to_string(file).expect("cannot read file");
        parse_code(&code);
    }

    fn parse_code(code: &str) {
        let pairs = parse_gt_code(code);

        if let Err(err) = pairs {
            println!("{}", err);
            assert!(false, "Failed to parse code");
        }
    }
}
