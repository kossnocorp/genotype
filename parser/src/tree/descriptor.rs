use super::{
    object::{parse_object, Object},
    primitive::{parse_primitive, Primitive},
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub enum Descriptor {
    Primitive(Primitive),
    Name(String),
    Object(Object),
}

pub fn parse_descriptor(pair: Pair<'_, Rule>) -> Result<Descriptor, Box<dyn std::error::Error>> {
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::primitive => return Ok(Descriptor::Primitive(parse_primitive(pair)?)),

            Rule::name => return Ok(Descriptor::Name(pair.as_str().to_string())),

            Rule::object => return Ok(Descriptor::Object(parse_object(pair)?)),

            _ => {
                println!("3 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        }
    }

    Err("Can't parse the descriptor".into())
}
