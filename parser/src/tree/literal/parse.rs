use pest::iterators::Pair;

use crate::{diagnostic::error::GTNodeParseError, parser::Rule};

use super::GTLiteral;

impl TryFrom<Pair<'_, Rule>> for GTLiteral {
    type Error = GTNodeParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pair = pair.into_inner().next().unwrap();
        Ok(match pair.as_rule() {
            Rule::literal_string => {
                GTLiteral::String(pair.into_inner().next().unwrap().as_str().into())
            }

            Rule::literal_integer => {
                GTLiteral::Integer(pair.as_str().replace("_", "").parse().unwrap())
            }

            Rule::literal_float => {
                GTLiteral::Float(pair.as_str().replace("_", "").parse().unwrap())
            }

            Rule::literal_boolean => GTLiteral::Boolean(pair.as_str().parse().unwrap()),

            _ => {
                println!("6 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        })
    }
}
