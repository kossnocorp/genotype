use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseError, GTNodeParseResult, GTSpan};

use super::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut descriptors = vec![];
        let mut inner = pair.into_inner();

        while let Some(pair) = inner.next() {
            let pair = pair.into_inner().next().unwrap();
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

            descriptors.push(descriptor);
        }

        match descriptors.as_slice() {
            [] => Err(GTNodeParseError::Internal(span, GTNode::Descriptor)),

            [descriptor] => Ok(descriptor.to_owned()),

            descriptors => Ok(GTDescriptor::Union(GTUnion {
                span,
                descriptors: descriptors.to_owned(),
            })),
        }
    }
}
