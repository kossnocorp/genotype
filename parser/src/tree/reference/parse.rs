use pest::iterators::Pair;

use crate::{
    parser::Rule,
    tree::{GTIdentifier, GTResolve},
};

use super::GTReference;

impl GTReference {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Self {
        let identifier: GTIdentifier = pair.as_str().into();
        resolve.references.insert(identifier.clone());
        GTReference(identifier)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use crate::tree::GTModule;

    #[test]
    fn test_parse_references() {
        let parse = GTModule::parse(
            r#"use user/User

            Author = {
              name: Name
              user: User
            }
            
            Name = string"#
                .into(),
        )
        .unwrap();
        assert_eq!(
            parse.resolve.references,
            HashSet::from_iter(vec!["Name".into(), "User".into()])
        );
    }
}
