use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseError, GTNodeParseResult, GTSpan};

use super::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> GTNodeParseResult<Self> {
        let nullable = pair.as_rule() == Rule::nullable_descriptor;

        let span: GTSpan = pair.as_span().into();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTNodeParseError::Internal(span.clone(), GTNode::Descriptor))?;

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

            _ => return Err(GTNodeParseError::Internal(span.clone(), GTNode::Descriptor)),
        };

        if nullable {
            Ok(GTDescriptor::Nullable(Box::new(descriptor)))
        } else {
            Ok(descriptor)
        }
    }
}
