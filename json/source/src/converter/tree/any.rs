use genotype_json_schema::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTDescriptor> for GtjAny {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        match self {
            GtjAny::GtjNull(null) => null.convert(context),
            GtjAny::GtjBoolean(boolean) => boolean.convert(context),
            GtjAny::GtjNumber(number) => number.convert(context),
            GtjAny::GtjString(string) => string.convert(context),
            GtjAny::GtjArray(array) => array.convert(context),
            GtjAny::GtjObject(object) => object.convert(context),
            GtjAny::GtjUnion(union) => union.convert(context),
            GtjAny::GtjLiteral(literal) => literal.convert(context),
            GtjAny::GtjTuple(tuple) => tuple.convert(context),
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
            kind: GtjNullKindNull,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Null(Default::default())),
            null.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_boolean() {
        let boolean = GtjBoolean {
            kind: GtjBooleanKindBoolean,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean(Default::default())),
            boolean.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_number() {
        let number = GtjNumber {
            kind: GtjNumberKindNumber,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Number(Default::default())),
            number.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_string() {
        let string = GtjString {
            kind: GtjStringKindString,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::String(Default::default())),
            string.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_array() {
        let array = GtjArray {
            kind: GtjArrayKindArray,
            descriptor: GtjAny::GtjNull(GtjNull {
                kind: GtjNullKindNull,
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
            array.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_object() {
        let object = GtjObject {
            kind: GtjObjectKindObject,
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
            object.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_union() {
        let union = GtjUnion {
            kind: GtjUnionKindUnion,
            name: None,
            doc: None,
            descriptors: vec![],
        };
        assert_eq!(
            GTDescriptor::Union(GTUnion {
                span: Default::default(),
                descriptors: vec![],
            }),
            union.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_literal() {
        let literal = GtjLiteral {
            kind: GtjLiteralKindLiteral,
            name: None,
            doc: None,
            value: GtjLiteralValue::String("Hello".into()),
        };
        assert_eq!(
            GTDescriptor::Literal(GTLiteral::String(Default::default(), "Hello".into())),
            literal.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_tuple() {
        let tuple = GtjTuple {
            kind: GtjTupleKindTuple,
            name: None,
            doc: None,
            descriptors: vec![],
        };
        assert_eq!(
            GTDescriptor::Tuple(GTTuple {
                span: Default::default(),
                descriptors: vec![],
            }),
            tuple.convert(&mut Default::default()),
        );
    }
}
