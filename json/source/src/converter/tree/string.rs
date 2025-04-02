use genotype_json_schema::json::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTPrimitive> for GtjString {
    fn convert(&self, _context: &mut GtjConvertContext) -> GTPrimitive {
        GTPrimitive::String(Default::default())
    }
}

impl GtjConvert<GTDescriptor> for GtjString {
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
        let string = GtjString {
            kind: GtjStringKindString,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::String(Default::default()),
            string.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
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
}
