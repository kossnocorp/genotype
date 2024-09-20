use super::descriptor::{parse_descriptor, Descriptor};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub struct Tuple {
    pub descriptors: Vec<Descriptor>,
}

pub fn parse_tuple(pair: Pair<'_, Rule>) -> Result<Tuple, Box<dyn std::error::Error>> {
    let mut tuple = Tuple {
        descriptors: vec![],
    };

    for pair in pair.into_inner() {
        let descriptor = parse_descriptor(pair)?;
        tuple.descriptors.push(descriptor);
    }

    Ok(tuple)
}
