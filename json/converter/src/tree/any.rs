use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTDescriptor> for GtjAny {
    fn to_tree(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        match self {
            GtjAny::GtjNull(null) => null.to_tree(context),
            GtjAny::GtjBoolean(boolean) => boolean.to_tree(context),
            GtjAny::GtjNumber(number) => number.to_tree(context),
            GtjAny::GtjString(string) => string.to_tree(context),
            GtjAny::GtjArray(array) => array.to_tree(context),
            GtjAny::GtjObject(object) => object.to_tree(context),
            GtjAny::GtjUnion(union) => union.to_tree(context),
            GtjAny::GtjLiteral(literal) => literal.to_tree(context),
            GtjAny::GtjTuple(tuple) => tuple.to_tree(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_null() {
        let null = GtjNull {
            r#type: GtjNullTypeNull,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Null(Default::default())),
            null.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_boolean() {
        let boolean = GtjBoolean {
            r#type: GtjBooleanTypeBoolean,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean(Default::default())),
            boolean.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_number() {
        let number = GtjNumber {
            r#type: GtjNumberTypeNumber,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Number(Default::default())),
            number.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_string() {
        let string = GtjString {
            r#type: GtjStringTypeString,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::String(Default::default())),
            string.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_array() {
        let array = GtjArray {
            r#type: GtjArrayTypeArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                r#type: GtjNullTypeNull,
                name: None,
                doc: None,
            }),
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Array(Box::new(GTArray {
                span: Default::default(),
                descriptor: GTPrimitive::Null(Default::default()).into()
            })),
            array.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_object() {
        let object = GtjObject {
            r#type: GtjObjectTypeObject,
            name: None,
            doc: None,
            properties: vec![],
        };
        assert_eq!(
            GTDescriptor::Object(GTObject {
                span: Default::default(),
                name: GTObjectName::Named(GTIdentifier(Default::default(), "Root".into())),
                extensions: vec![],
                properties: vec![],
            }),
            object.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_union() {
        let union = GtjUnion {
            r#type: GtjUnionTypeUnion,
            name: None,
            doc: None,
            descriptors: vec![],
        };
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![],
            }),
            union.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_literal() {
        let literal = GtjLiteral {
            r#type: GtjLiteralTypeLiteral,
            name: None,
            doc: None,
            value: GtjLiteralValue::String("Hello".into()),
        };
        assert_eq!(
            GTDescriptor::Literal(GTLiteral::String(Default::default(), "Hello".into())),
            literal.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_tuple() {
        let tuple = GtjTuple {
            r#type: GtjTupleTypeTuple,
            name: None,
            doc: None,
            descriptors: vec![],
        };
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![],
            }),
            tuple.to_tree(&mut Default::default()),
        );
    }
}
