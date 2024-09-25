use super::descriptor::{parse_descriptor, Descriptor};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub descriptor: Descriptor,
}

pub fn parse_array(pair: Pair<'_, Rule>) -> Result<Array, Box<dyn std::error::Error>> {
    let pair = pair.into_inner().next().unwrap(); // [TODO]
    let descriptor = parse_descriptor(pair)?;

    Ok(Array { descriptor })
}
