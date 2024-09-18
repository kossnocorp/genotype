use crate::parser::Rule;

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Boolean,
    String,
    Int,
    Float,
}

pub fn parse_primitive(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<Primitive, Box<dyn std::error::Error>> {
    match pair.as_str() {
        "boolean" => Ok(Primitive::Boolean),

        "string" => Ok(Primitive::String),

        "int" => Ok(Primitive::Int),

        "float" => Ok(Primitive::Float),

        _ => Err("Unknown primitive".into()),
    }
}
