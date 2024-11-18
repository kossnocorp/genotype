use pest::iterators::Pair;

use crate::{parser::Rule, tree::GTIdentifier, GTContext};

use super::GTReference;

impl GTReference {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Self {
        let span = pair.as_span().into();
        let identifier: GTIdentifier = pair.into();
        context.resolve.references.insert(identifier.clone());
        GTReference(span, identifier)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use miette::NamedSource;
    use pretty_assertions::assert_eq;

    use crate::{tree::GTModule, GTIdentifier};

    #[test]
    fn test_parse_references() {
        let parse = GTModule::parse(
            "module".into(),
            NamedSource::new(
                "module.type",
                r#"use user/User

            Author = {
              name: Name
              user: User
            }
            
            Name = string"#
                    .into(),
            ),
        )
        .unwrap();
        assert_eq!(
            parse.resolve.references,
            HashSet::from_iter(vec![
                GTIdentifier::new((58, 62).into(), "Name".into()),
                GTIdentifier::new((83, 87).into(), "User".into())
            ])
        );
    }
}
