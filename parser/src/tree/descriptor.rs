use super::{
    object::{parse_object, Object},
    primitive::{parse_primitive, Primitive},
    union::Union,
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub enum Descriptor {
    Primitive(Primitive),
    Name(String),
    Object(Object),
    Nullable(Box<Descriptor>),
    // [TODO]
    // Union(Union),
}

pub fn parse_descriptor(pair: Pair<'_, Rule>) -> Result<Descriptor, Box<dyn std::error::Error>> {
    let nullable = pair.as_rule() == Rule::nullable_descriptor;
    let pair = pair.into_inner().next().unwrap(); // [TODO]

    let descriptor = match pair.as_rule() {
        Rule::primitive => Descriptor::Primitive(parse_primitive(pair)?),

        Rule::name => Descriptor::Name(pair.as_str().to_string()),

        Rule::object => Descriptor::Object(parse_object(pair)?),

        Rule::descriptor => parse_descriptor(pair)?,

        _ => {
            println!("3 ====== unknown rule: {:?}", pair);
            unreachable!("unknown rule");
        }
    };

    if nullable {
        Ok(Descriptor::Nullable(Box::new(descriptor)))
    } else {
        Ok(descriptor)
    }
}
