use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseResult, GTParseError, GTSpan};

use super::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut descriptors = vec![];
        let mut inner = pair.into_inner();

        let is_union = inner.len() > 1;
        if is_union {
            context.parents.push(GTContextParent::Anonymous);
        }

        while let Some(pair) = inner.next() {
            let pair = pair.into_inner().next().unwrap();
            let descriptor = match pair.as_rule() {
                Rule::primitive => GTDescriptor::Primitive(pair.try_into()?),

                Rule::name => GTDescriptor::Reference(GTReference::parse(pair, context)),

                Rule::object => GTDescriptor::Object(GTObject::parse(pair, context)?),

                Rule::array => GTDescriptor::Array(Box::new(GTArray::parse(pair, context)?)),

                Rule::tuple => GTDescriptor::Tuple(GTTuple::parse(pair, context)?),

                Rule::descriptor => GTDescriptor::parse(pair, context)?,

                Rule::alias => GTDescriptor::Alias(Box::new(GTAlias::parse(pair, context)?)),

                Rule::inline_import => {
                    GTDescriptor::InlineImport(GTInlineImport::parse(pair, context)?)
                }

                Rule::literal => GTDescriptor::Literal(pair.try_into()?),

                Rule::record => GTDescriptor::Record(Box::new(GTRecord::parse(pair, context)?)),

                Rule::any => GTDescriptor::Any(pair.into()),

                _ => return Err(GTParseError::UnknownRule(span.clone(), GTNode::Descriptor)),
            };

            descriptors.push(descriptor);
        }

        if is_union {
            context.pop_parent(span.clone(), GTNode::Descriptor)?;
        }

        match descriptors.as_slice() {
            [] => Err(GTParseError::InternalMessage(
                span,
                GTNode::Descriptor,
                "no descriptors found",
            )),

            [descriptor] => Ok(descriptor.to_owned()),

            descriptors => Ok(GTDescriptor::Union(GTUnion {
                span,
                descriptors: descriptors.to_owned(),
            })),
        }
    }
}
