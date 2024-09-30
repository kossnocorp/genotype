use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTObject;

impl TryFrom<Pair<'_, Rule>> for GTObject {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut object = GTObject { properties: vec![] };

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::required_property | Rule::optional_property => {
                    object.properties.push(pair.try_into()?);
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
