use std::primitive;

use super::{
    alias::{parse_alias, Alias},
    object::{self, parse_object, Object},
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

pub fn parse_descriptor(
    pair: Pair<'_, Rule>,
) -> Result<(Descriptor, Vec<Alias>), Box<dyn std::error::Error>> {
    let nullable = pair.as_rule() == Rule::nullable_descriptor;
    let pair = pair.into_inner().next().unwrap(); // [TODO]

    let (descriptor, hoisted) = match pair.as_rule() {
        Rule::primitive => {
            let primitive = parse_primitive(pair)?;
            (Descriptor::Primitive(primitive), vec![])
        }

        Rule::name => {
            let name = pair.as_str().to_string();
            (Descriptor::Name(name), vec![])
        }

        Rule::object => {
            let (object, hoisted) = parse_object(pair)?;
            (Descriptor::Object(object), hoisted)
        }

        Rule::descriptor => parse_descriptor(pair)?,

        // When we have an alias in place of a descriptor, we need to parse it and hoist it up
        // to the module level.
        Rule::alias => {
            let (alias, alias_hoisted) = parse_alias(pair)?;
            // [TODO] Figure out how I can use &str instead of String
            let name = alias.name.clone();
            let mut hoisted = vec![alias];
            hoisted.extend(alias_hoisted);
            (Descriptor::Name(name), hoisted)
        }

        _ => {
            println!("3 ====== unknown rule: {:?}", pair);
            unreachable!("unknown rule");
        }
    };

    if nullable {
        Ok((Descriptor::Nullable(Box::new(descriptor)), hoisted))
    } else {
        Ok((descriptor, hoisted))
    }
}
