use super::descriptor::{parse_descriptor, GTDescriptor};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone)]
pub struct GTArray {
    pub descriptor: GTDescriptor,
}

pub fn parse_array(pair: Pair<'_, Rule>) -> Result<GTArray, Box<dyn std::error::Error>> {
    let pair = pair.into_inner().next().unwrap(); // [TODO]
    let descriptor = parse_descriptor(pair)?;

    Ok(GTArray { descriptor })
}
