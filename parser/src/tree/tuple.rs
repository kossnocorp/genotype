use super::{
    alias::Alias,
    descriptor::{self, parse_descriptor, Descriptor},
    primitive::Primitive,
    property::{parse_property, Property},
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub struct Tuple {
    pub descriptors: Vec<Descriptor>,
}

pub fn parse_tuple(
    pair: Pair<'_, Rule>,
) -> Result<(Tuple, Vec<Alias>), Box<dyn std::error::Error>> {
    let mut tuple = Tuple {
        descriptors: vec![],
    };
    let mut hoisted = vec![];

    for pair in pair.into_inner() {
        let (descriptor, descriptor_hoisted) = parse_descriptor(pair)?;
        hoisted.extend(descriptor_hoisted);
        tuple.descriptors.push(descriptor);
    }

    Ok((tuple, hoisted))
}
