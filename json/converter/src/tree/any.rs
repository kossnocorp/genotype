use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTDescriptor> for GtjAny {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        match self {
            GtjAny::GtjBoolean(boolean) => boolean.to_tree_with_context(context),
            GtjAny::GtjNumber(number) => number.to_tree_with_context(context),
            GtjAny::GtjString(string) => string.to_tree_with_context(context),
            GtjAny::GtjArray(array) => array.to_tree_with_context(context),
            GtjAny::GtjObject(object) => object.to_tree_with_context(context),
            GtjAny::GtjUnion(union) => union.to_tree_with_context(context),
            GtjAny::GtjLiteral(literal) => literal.to_tree_with_context(context),
            GtjAny::GtjTuple(tuple) => tuple.to_tree_with_context(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_boolean() {
        let boolean = GtjBoolean {
            r#type: GtjBooleanTypeBoolean,
            name: None,
            doc: None,
        };
        let tree: GTDescriptor = boolean.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @"
        Primitive(GTPrimitive(
          span: GTSpan(0, 0),
          kind: Boolean,
          doc: None,
          attributes: [],
        ))
        ");
    }

    #[test]
    fn test_convert_number() {
        let number = GtjNumber {
            r#type: GtjNumberTypeNumber,
            name: None,
            doc: None,
        };

        let tree: GTDescriptor = number.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @"
        Primitive(GTPrimitive(
          span: GTSpan(0, 0),
          kind: Number,
          doc: None,
          attributes: [],
        ))
        ");
    }

    #[test]
    fn test_convert_string() {
        let string = GtjString {
            r#type: GtjStringTypeString,
            name: None,
            doc: None,
        };

        let tree: GTDescriptor = string.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @"
        Primitive(GTPrimitive(
          span: GTSpan(0, 0),
          kind: String,
          doc: None,
          attributes: [],
        ))
        ");
    }

    #[test]
    fn test_convert_array() {
        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNumber(GtjNumber {
                name: None,
                doc: None,
                r#type: GtjNumberTypeNumber,
            }),
            name: None,
            doc: None,
        };

        let tree: GTDescriptor = array.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @"
        Array(GTArray(
          span: GTSpan(0, 0),
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(0, 0),
            kind: Number,
            doc: None,
            attributes: [],
          )),
        ))
        ");
    }

    #[test]
    fn test_convert_object() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            name: None,
            doc: None,
            properties: vec![],
        };

        let tree: GTDescriptor = object.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @r#"
        Object(GTObject(
          span: GTSpan(0, 0),
          name: Named(GTIdentifier(GTSpan(0, 0), "Root")),
          extensions: [],
          properties: [],
        ))
        "#);
    }

    #[test]
    fn test_convert_union() {
        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            name: None,
            doc: None,
            descriptors: vec![],
        };

        let tree: GTDescriptor = union.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @"
        Union(GTUnion(
          span: GTSpan(0, 0),
          descriptors: [],
        ))
        ");
    }

    #[test]
    fn test_convert_literal() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: None,
            doc: None,
            value: GtjLiteralValue::String("Hello".into()),
        };

        let tree: GTDescriptor = literal.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @r#"
        Literal(GTLiteral(
          span: GTSpan(0, 0),
          doc: None,
          attributes: [],
          value: String("Hello"),
        ))
        "#);
    }

    #[test]
    fn test_convert_tuple() {
        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            name: None,
            doc: None,
            descriptors: vec![],
        };

        let tree: GTDescriptor = tuple.to_tree_with_context(&mut Default::default());
        assert_ron_snapshot!(tree, @"
        Tuple(GTTuple(
          span: GTSpan(0, 0),
          descriptors: [],
        ))
        ");
    }
}
