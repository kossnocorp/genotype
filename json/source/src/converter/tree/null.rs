use genotype_json_schema::json::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTPrimitive> for GtjNull {
    fn convert(&self, _context: &mut GtjConvertContext) -> GTPrimitive {
        GTPrimitive::Null(Default::default())
    }
}

impl GtjConvert<GTDescriptor> for GtjNull {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Primitive(self.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let null = GtjNull {
            kind: GtjNullKindNull,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::Null(Default::default()),
            null.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
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
}
