use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTPrimitive;

impl TryFrom<Pair<'_, Rule>> for GTPrimitive {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_str() {
            "boolean" => Ok(GTPrimitive::Boolean),

            "string" => Ok(GTPrimitive::String),

            "int" => Ok(GTPrimitive::Int),

            "float" => Ok(GTPrimitive::Float),

            _ => Err("Unknown primitive".into()),
        }
    }
}
