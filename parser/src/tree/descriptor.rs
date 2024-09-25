use super::{
    alias::{parse_alias, Alias},
    array::{parse_array, Array},
    object::{parse_object, Object},
    primitive::{parse_primitive, Primitive},
    reference::{parse_reference, Reference},
    tuple::{parse_tuple, Tuple},
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone)]
pub enum Descriptor {
    Alias(Box<Alias>),
    Primitive(Primitive),
    Name(String),
    Object(Object),
    Array(Box<Array>),
    Tuple(Tuple),
    Reference(Reference),
    Nullable(Box<Descriptor>),
    // [TODO]
    // Union(Union),
}

pub fn parse_descriptor(pair: Pair<'_, Rule>) -> Result<Descriptor, Box<dyn std::error::Error>> {
    let nullable = pair.as_rule() == Rule::nullable_descriptor;
    let pair = pair.into_inner().next().unwrap(); // [TODO]

    let descriptor = match pair.as_rule() {
        Rule::primitive => {
            let primitive = parse_primitive(pair)?;
            Descriptor::Primitive(primitive)
        }

        Rule::name => {
            let name = pair.as_str().to_string();
            Descriptor::Name(name)
        }

        Rule::object => {
            let object = parse_object(pair)?;
            Descriptor::Object(object)
        }

        Rule::array => {
            let array = parse_array(pair)?;
            Descriptor::Array(Box::new(array))
        }

        Rule::tuple => {
            let tuple = parse_tuple(pair)?;
            Descriptor::Tuple(tuple)
        }

        Rule::descriptor => parse_descriptor(pair)?,

        Rule::alias => {
            let alias = parse_alias(pair)?;
            Descriptor::Alias(Box::new(alias))
        }

        Rule::inline_reference => {
            let reference = parse_reference(pair)?;
            Descriptor::Reference(reference)
        }

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
