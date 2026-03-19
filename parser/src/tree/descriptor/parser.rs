use crate::prelude::internal::*;

impl GTDescriptor {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut descriptors = vec![];

        let inner = pair.into_inner();

        let is_union = inner.len() > 1;
        if is_union {
            context.enter_parent(GTContextParent::Anonymous);
        }

        for pair in inner {
            let mut descriptor_inner = pair.into_inner();
            let next_pair = descriptor_inner
                .next()
                .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::Descriptor))?;

            let descriptor = parse(
                descriptor_inner,
                next_pair,
                ParseState::Annotation(span.clone(), None, vec![]),
                context,
            )?;

            descriptors.push(descriptor);
        }

        if is_union {
            context.exit_parent(span.clone(), GTNode::Descriptor)?;
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

fn parse(
    mut inner: Pairs<'_, Rule>,
    pair: Pair<'_, Rule>,
    state: ParseState,
    context: &mut GTContext,
) -> GTNodeParseResult<GTDescriptor> {
    match state {
        ParseState::Annotation(span, doc_acc, mut attributes) => match pair.as_rule() {
            Rule::line_doc => {
                let doc = pair.into_inner().find(|p| p.as_rule() == Rule::doc);
                let doc_acc = if let Some(pair) = doc {
                    Some(if let Some(doc) = doc_acc {
                        doc.concat(pair)
                    } else {
                        pair.into()
                    })
                } else {
                    doc_acc
                };

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        ParseState::Annotation(span, doc_acc, attributes),
                        context,
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Descriptor)),
                }
            }

            Rule::attribute => {
                let attribute = GTAttribute::parse(pair, context)?;
                attributes.push(attribute);

                match inner.next() {
                    Some(pair) => parse(
                        inner,
                        pair,
                        ParseState::Annotation(span, doc_acc, attributes),
                        context,
                    ),
                    None => Err(GTParseError::Internal(span, GTNode::Descriptor)),
                }
            }

            _ => parse(
                inner,
                pair,
                ParseState::Descriptor(span, doc_acc, attributes),
                context,
            ),
        },

        ParseState::Descriptor(span, doc, attributes) => {
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

                Rule::literal => {
                    context.provide_annotation(GTContextAnnotation { doc, attributes });
                    GTDescriptor::Literal(GTLiteral::parse(pair, context)?)
                }

                Rule::record => GTDescriptor::Record(Box::new(GTRecord::parse(pair, context)?)),

                Rule::any => GTDescriptor::Any(pair.into()),

                Rule::branded => GTDescriptor::Branded(GTBranded::parse(pair, context)?),

                rule => {
                    return Err(GTParseError::UnexpectedRule(
                        span.clone(),
                        GTNode::Descriptor,
                        rule,
                    ));
                }
            };

            Ok(descriptor)
        }
    }
}

enum ParseState {
    Annotation(GTSpan, Option<GTDoc>, Vec<GTAttribute>),
    Descriptor(GTSpan, Option<GTDoc>, Vec<GTAttribute>),
}
