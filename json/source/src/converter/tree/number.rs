use genotype_json_schema::json::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTPrimitive> for GtjNumber {
    fn convert(&self, _context: &mut GtjConvertContext) -> GTPrimitive {
        GTPrimitive::Number(Default::default())
    }
}

impl GtjConvert<GTDescriptor> for GtjNumber {
    fn convert(&self, context: &mut GtjConvertContext) -> GTDescriptor {
        GTDescriptor::Primitive(self.convert(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_json_schema::json::GtjNumberKindNumber;
    use genotype_parser::GTDescriptor;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let number = GtjNumber {
            kind: GtjNumberKindNumber,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::Number(Default::default()),
            number.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
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
}
