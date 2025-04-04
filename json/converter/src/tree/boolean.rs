use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjBoolean {
    fn to_tree(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive::Boolean(Default::default())
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjBoolean {
    fn to_tree(&self, context: &mut GtjTreeConvertContext) -> GTDescriptor {
        GTDescriptor::Primitive(self.to_tree(context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let boolean = GtjBoolean {
            r#type: GtjBooleanTypeBoolean,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::Boolean(Default::default()),
            boolean.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
        let boolean = GtjBoolean {
            r#type: GtjBooleanTypeBoolean,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTDescriptor::Primitive(GTPrimitive::Boolean(Default::default())),
            boolean.to_tree(&mut Default::default())
        );
    }
}
