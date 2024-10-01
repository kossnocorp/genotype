use pest::iterators::Pair;

use crate::{parser::Rule, tree::name::GTName};

use super::GTInlineImport;

impl TryFrom<Pair<'_, Rule>> for GTInlineImport {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let str = pair.as_str().to_string();

        // [TODO]
        let name_index = str.rfind("/").unwrap();
        let path = &str[..name_index];
        let name = &str[name_index + 1..];

        Ok(GTInlineImport {
            path: path.to_string(),
            name: GTName(name.to_string()),
        })
    }
}