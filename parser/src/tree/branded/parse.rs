use pest::iterators::Pair;

use crate::{
    parser::Rule, GTContext, GTNode, GTNodeParseResult, GTParseError, GTPrimitive, GTSpan,
};

use super::GTBranded;

impl GTBranded {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Array))?;
        let primitive: GTPrimitive = pair.try_into()?;
        let name = context.get_name(&span, &primitive.to_string());
        let id = context.get_definition_id(&name);

        Ok(GTBranded {
            span,
            id,
            name,
            primitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::branded, "@int").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTBranded {
                span: GTSpan(0, 4),
                id: GTDefinitionId("module".into(), "I64".into()),
                name: GTIdentifier::new(GTSpan(0, 4), "I64".into()),
                primitive: GTPrimitive::Int64(GTSpan(1, 4)),
            }
        );
    }

    #[test]
    fn test_alias() {
        let mut pairs = GenotypeParser::parse(Rule::branded, "@int").unwrap();
        let mut context = GTContext::new("module".into());
        context.enter_parent(GTContextParent::Alias(GTIdentifier::new(
            GTSpan(0, 3),
            "Id".into(),
        )));
        assert_eq!(
            GTBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTBranded {
                span: GTSpan(0, 4),
                id: GTDefinitionId("module".into(), "Id".into()),
                name: GTIdentifier::new(GTSpan(0, 3), "Id".into()),
                primitive: GTPrimitive::Int64(GTSpan(1, 4)),
            }
        );
    }

    #[test]
    fn test_anonymous() {
        let mut pairs = GenotypeParser::parse(Rule::branded, "@int").unwrap();
        let mut context = GTContext::new("module".into());
        context.enter_parent(GTContextParent::Alias(GTIdentifier::new(
            GTSpan(0, 3),
            "Id".into(),
        )));
        context.enter_parent(GTContextParent::Anonymous);
        assert_eq!(
            GTBranded::parse(pairs.next().unwrap(), &mut context).unwrap(),
            GTBranded {
                span: GTSpan(0, 4),
                id: GTDefinitionId("module".into(), "IdI64".into()),
                name: GTIdentifier::new(GTSpan(0, 4), "IdI64".into()),
                primitive: GTPrimitive::Int64(GTSpan(1, 4)),
            }
        );
    }
}
