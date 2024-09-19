use super::{
    alias::Alias,
    descriptor::{self, parse_descriptor, Descriptor},
    primitive::Primitive,
    property::{parse_property, Property},
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub struct Array {
    pub descriptor: Descriptor,
}

pub fn parse_array(
    pair: Pair<'_, Rule>,
) -> Result<(Array, Vec<Alias>), Box<dyn std::error::Error>> {
    let pair = pair.into_inner().next().unwrap(); // [TODO]
    let (descriptor, hoisted) = parse_descriptor(pair)?;

    Ok((Array { descriptor }, hoisted))
}
