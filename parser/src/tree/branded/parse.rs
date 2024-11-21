use pest::iterators::Pair;

use crate::{
    parser::Rule, GTContext, GTDefinitionId, GTIdentifier, GTNode, GTNodeParseResult, GTParseError,
    GTSpan,
};

use super::GTBranded;

impl GTBranded {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let branded = match pair.as_str() {
            "@boolean" => {
                let (id, name) = get_id_and_name(context, &span, "Bool");
                GTBranded::Boolean(span, id, name)
            }

            "@string" => {
                let (id, name) = get_id_and_name(context, &span, "Str");
                GTBranded::String(span, id, name)
            }

            "@int" => {
                let (id, name) = get_id_and_name(context, &span, "Int");
                GTBranded::Int(span, id, name)
            }

            "@float" => {
                let (id, name) = get_id_and_name(context, &span, "Float");
                GTBranded::Float(span, id, name)
            }

            "@null" => {
                let (id, name) = get_id_and_name(context, &span, "Null");
                GTBranded::Null(span, id, name)
            }

            _ => {
                return Err(GTParseError::Internal(span, GTNode::Branded));
            }
        };

        Ok(branded)
    }
}

fn get_id_and_name(
    context: &mut GTContext,
    span: &GTSpan,
    base_name: &str,
) -> (GTDefinitionId, GTIdentifier) {
    let name = context.get_name(span, base_name);
    let id = context.get_definition_id(&name);
    (id, name)
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
            GTBranded::Int(
                GTSpan(0, 4),
                GTDefinitionId("module".into(), "Int".into()),
                GTIdentifier::new(GTSpan(0, 4), "Int".into())
            )
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
            GTBranded::Int(
                GTSpan(0, 4),
                GTDefinitionId("module".into(), "Id".into()),
                GTIdentifier::new(GTSpan(0, 3), "Id".into())
            )
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
            GTBranded::Int(
                GTSpan(0, 4),
                GTDefinitionId("module".into(), "IdInt".into()),
                GTIdentifier::new(GTSpan(0, 4), "IdInt".into())
            )
        );
    }
}
