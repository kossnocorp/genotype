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

        Ok(GTInlineImport {
            path: GTPath(path.into()),
            name: GTIdentifier(name.into()),
        })
    }
}
