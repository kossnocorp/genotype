use super::{
    alias::{parse_alias, GTAlias},
    array::{parse_array, GTArray},
    object::{parse_object, GTObject},
    primitive::{parse_primitive, GTPrimitive},
    reference::{parse_reference, GTReference},
    tuple::{parse_tuple, GTTuple},
};
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Clone)]
pub enum GTDescriptor {
    Alias(Box<GTAlias>),
    Primitive(GTPrimitive),
    Name(String),
    Object(GTObject),
    Array(Box<GTArray>),
    Tuple(GTTuple),
    Reference(GTReference),
    Nullable(Box<GTDescriptor>),
    // [TODO]
    // Union(Union),
}

pub fn parse_descriptor(pair: Pair<'_, Rule>) -> Result<GTDescriptor, Box<dyn std::error::Error>> {
    let nullable = pair.as_rule() == Rule::nullable_descriptor;
    let pair = pair.into_inner().next().unwrap(); // [TODO]

    let descriptor = match pair.as_rule() {
        Rule::primitive => {
            let primitive = parse_primitive(pair)?;
            GTDescriptor::Primitive(primitive)
        }

        Rule::name => {
            let name = pair.as_str().to_string();
            GTDescriptor::Name(name)
        }

        Rule::object => {
            let object = parse_object(pair)?;
            GTDescriptor::Object(object)
        }

        Rule::array => {
            let array = parse_array(pair)?;
            GTDescriptor::Array(Box::new(array))
        }

        Rule::tuple => {
            let tuple = parse_tuple(pair)?;
            GTDescriptor::Tuple(tuple)
        }

        Rule::descriptor => parse_descriptor(pair)?,

        Rule::alias => {
            let alias = parse_alias(pair)?;
            GTDescriptor::Alias(Box::new(alias))
        }

        Rule::inline_reference => {
            let reference = parse_reference(pair)?;
            GTDescriptor::Reference(reference)
        }

        _ => {
            println!("3 ====== unknown rule: {:?}", pair);
            unreachable!("unknown rule");
        }
    };

    if nullable {
        Ok(GTDescriptor::Nullable(Box::new(descriptor)))
    } else {
        Ok(descriptor)
    }
}
