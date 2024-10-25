use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTAny;

impl From<Pair<'_, Rule>> for GTAny {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let span = pair.as_span().into();
        GTAny(span)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;

    #[test]
    fn test_from_pair() {
        let mut pairs = GenotypeParser::parse(Rule::any, "any").unwrap();
        assert_eq!(
            GTAny::try_from(pairs.next().unwrap()).unwrap(),
            GTAny(GTSpan(0, 3))
        );
    }
}
