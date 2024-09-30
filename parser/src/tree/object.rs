use super::property::{parse_property, GTProperty};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone)]
pub struct GTObject {
    pub properties: Vec<GTProperty>,
}

pub fn parse_object(pair: Pair<'_, Rule>) -> Result<GTObject, Box<dyn std::error::Error>> {
    let mut object = GTObject { properties: vec![] };

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::required_property | Rule::optional_property => {
                let property = parse_property(pair)?;
                object.properties.push(property);
            }

            _ => {
                println!("4 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        }
    }

    Ok(object)
}
