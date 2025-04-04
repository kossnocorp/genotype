use genotype_json_types::*;
use genotype_parser::*;

use crate::{GtjTreeConvert, GtjTreeConvertContext};

impl GtjTreeConvert<GTPrimitive> for GtjNumber {
    fn to_tree(&self, _context: &mut GtjTreeConvertContext) -> GTPrimitive {
        GTPrimitive::Number(Default::default())
    }
}

impl GtjTreeConvert<GTDescriptor> for GtjNumber {
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
        let number = GtjNumber {
            r#type: GtjNumberTypeNumber,
            name: None,
            doc: None,
        };
        assert_eq!(
            GTPrimitive::Number(Default::default()),
            number.to_tree(&mut Default::default()),
        );
    }

    #[test]
    fn test_convert_descriptor() {
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
}
