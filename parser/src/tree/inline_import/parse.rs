use pest::iterators::Pair;

use crate::{
    parser::Rule,
    tree::{identifier::GTIdentifier, path::GTPath, GTResolve},
};

use super::GTInlineImport;

impl GTInlineImport {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let str = pair.as_str().to_string();

        // [TODO]
        let name_index = str.rfind("/").unwrap();
        let path = &str[..name_index];
        let name = &str[name_index + 1..];

        let path = GTPath::new(path);
        resolve.deps.insert(path.clone());

        Ok(GTInlineImport {
            path,
            name: GTIdentifier(name.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use crate::tree::GTModule;

    #[test]
    fn test_parse_deps_base() {
        let code = r#"Order = {
            book: book/Book
            user: ./misc/user/User
        }"#;
        let parse = GTModule::parse(code.into()).unwrap();
        assert_eq!(
            parse.resolve.deps,
            HashSet::from_iter(vec!["book".into(), "./misc/user".into(),])
        );
    }

    #[test]
    fn test_parse_deps_normalize() {
        let code = r#"Order = {
            book: book/Book
            user: ./misc/../misc/./user/User
        }"#;
        let parse = GTModule::parse(code.into()).unwrap();
        assert_eq!(
            parse.resolve.deps,
            HashSet::from_iter(vec!["book".into(), "./misc/user".into(),])
        );
    }
}
