use super::{
    alias::Alias,
    property::{parse_property, Property},
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub properties: Vec<Property>,
}

pub fn parse_object(
    pair: Pair<'_, Rule>,
) -> Result<(Object, Vec<Alias>), Box<dyn std::error::Error>> {
    let mut object = Object { properties: vec![] };
    let mut hoisted = vec![];

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::required_property | Rule::optional_property => {
                let (property, property_hoisted) = parse_property(pair)?;
                hoisted.extend(property_hoisted);
                object.properties.push(property);
            }

            _ => {
                println!("4 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        }
    }

    Ok((object, hoisted))
}
