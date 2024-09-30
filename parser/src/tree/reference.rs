use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug, PartialEq, Clone)]
pub struct GTReference {
    pub path: String,
    pub name: String,
}

pub fn parse_reference(pair: Pair<'_, Rule>) -> Result<GTReference, Box<dyn std::error::Error>> {
    let str = pair.as_str().to_string();

    // [TODO]
    let name_index = str.rfind("/").unwrap();
    let path = &str[..name_index];
    let name = &str[name_index + 1..];

    Ok(GTReference {
        path: path.to_string(),
        name: name.to_string(),
    })
}
