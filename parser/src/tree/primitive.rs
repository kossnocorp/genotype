use crate::parser::Rule;

#[derive(Debug, PartialEq, Clone)]
pub enum GTPrimitive {
    Boolean,
    String,
    Int,
    Float,
}

pub fn parse_primitive(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<GTPrimitive, Box<dyn std::error::Error>> {
    match pair.as_str() {
        "boolean" => Ok(GTPrimitive::Boolean),

        "string" => Ok(GTPrimitive::String),

        "int" => Ok(GTPrimitive::Int),

        "float" => Ok(GTPrimitive::Float),

        _ => Err("Unknown primitive".into()),
    }
}
