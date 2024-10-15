use pest::iterators::Pair;

use crate::{
    parser::Rule,
    tree::{GTIdentifier, GTResolve},
};

use super::GTReference;

impl GTReference {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Self {
        let span = pair.as_span().into();
        let identifier: GTIdentifier = pair.into();
        resolve.references.insert(identifier.clone());
        GTReference(span, identifier)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use crate::{tree::GTModule, GTIdentifier, GTSourceCode};

    #[test]
    fn test_parse_references() {
        let parse = GTModule::parse(GTSourceCode::new(
            "module.type".into(),
            r#"use user/User

            Author = {
              name: Name
              user: User
            }
            
            Name = string"#
                .into(),
        ))
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
