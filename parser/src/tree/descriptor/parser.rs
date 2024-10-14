use pest::iterators::Pair;

use crate::{diagnostic::error::GTNodeParseError, parser::Rule};

use super::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Result<Self, GTNodeParseError> {
        let nullable = pair.as_rule() == Rule::nullable_descriptor;
        let pair = pair.into_inner().next().unwrap(); // [TODO]

        let descriptor = match pair.as_rule() {
            Rule::primitive => GTDescriptor::Primitive(pair.try_into()?),

            Rule::name => GTDescriptor::Reference(GTReference::parse(pair, resolve)),

            Rule::object => GTDescriptor::Object(GTObject::parse(pair, resolve)?),

            Rule::array => GTDescriptor::Array(Box::new(GTArray::parse(pair, resolve)?)),

            Rule::tuple => GTDescriptor::Tuple(GTTuple::parse(pair, resolve)?),

            Rule::descriptor => GTDescriptor::parse(pair, resolve)?,

            Rule::alias => GTDescriptor::Alias(Box::new(GTAlias::parse(pair, resolve)?)),

            Rule::inline_import => {
                GTDescriptor::InlineImport(GTInlineImport::parse(pair, resolve)?)
            }

            Rule::literal => GTDescriptor::Literal(pair.try_into()?),

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
}
