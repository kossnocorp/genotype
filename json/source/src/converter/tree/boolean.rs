use genotype_json_schema::json::*;
use genotype_parser::*;

use crate::{GtjConvert, GtjConvertContext};

impl GtjConvert<GTPrimitive> for GtjBoolean {
    fn convert(&self, _context: &mut GtjConvertContext) -> GTPrimitive {
        GTPrimitive::Boolean(Default::default())
    }
}

impl GtjConvert<GTDescriptor> for GtjBoolean {
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
        let boolean = GtjBoolean {
            kind: GtjBooleanKindBoolean,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::Boolean(Default::default()),
            boolean.convert(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
        let boolean = GtjBoolean {
            kind: GtjBooleanKindBoolean,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean(Default::default())),
            boolean.convert(&mut Default::default())
        );
    }
}
