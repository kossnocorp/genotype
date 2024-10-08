use pest::iterators::Pair;

use crate::{
    parser::Rule,
    tree::{GTProperty, GTResolve},
};

use super::GTObject;

impl GTObject {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut object = GTObject { properties: vec![] };

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::required_property | Rule::optional_property => {
                    object.properties.push(GTProperty::parse(pair, resolve)?);
                }

                _ => {
                    println!("4 ====== unknown rule: {:?}", pair);
                    unreachable!("unknown rule");
                }
            }
        }

        Ok(object)
    }
}
