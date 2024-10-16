use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseError, GTNodeParseResult, GTSpan};

use super::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTNodeParseError::Internal(span.clone(), GTNode::Descriptor))?;

        match pair.as_rule() {
            Rule::primitive => Ok(GTDescriptor::Primitive(pair.try_into()?)),

            Rule::name => Ok(GTDescriptor::Reference(GTReference::parse(pair, resolve))),

            Rule::object => Ok(GTDescriptor::Object(GTObject::parse(pair, resolve)?)),

            Rule::array => Ok(GTDescriptor::Array(Box::new(GTArray::parse(
                pair, resolve,
            )?))),

            Rule::tuple => Ok(GTDescriptor::Tuple(GTTuple::parse(pair, resolve)?)),

            Rule::descriptor => Ok(GTDescriptor::parse(pair, resolve)?),

            Rule::alias => Ok(GTDescriptor::Alias(Box::new(GTAlias::parse(
                pair, resolve,
            )?))),

            Rule::inline_import => Ok(GTDescriptor::InlineImport(GTInlineImport::parse(
                pair, resolve,
            )?)),

            Rule::literal => Ok(GTDescriptor::Literal(pair.try_into()?)),

            _ => Err(GTNodeParseError::Internal(span.clone(), GTNode::Descriptor)),
        }
    }
}
