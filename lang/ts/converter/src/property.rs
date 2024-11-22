use genotype_lang_ts_tree::*;
use genotype_parser::tree::property::GTProperty;

use crate::{context::TSConvertContext, convert::TSConvert};

impl TSConvert<TSProperty> for GTProperty {
    fn convert(&self, context: &mut TSConvertContext) -> TSProperty {
        let descriptor = self.descriptor.convert(context);

        let descriptor = if self.required {
            descriptor
        } else {
            TSUnion {
                descriptors: vec![descriptor, TSPrimitive::Undefined.into()],
            }
            .into()
        };

        TSProperty {
            doc: self.doc.as_ref().map(|d| d.convert(context)),
            name: self.name.convert(context),
            descriptor,
            required: self.required,
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: true,
            }
            .convert(&mut Default::default()),
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true,
            }
        );
    }

    #[test]
    fn test_convert_doc() {
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: true,
            }
            .convert(&mut Default::default()),
            TSProperty {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true,
            }
        );
    }

    #[test]
    fn test_convert_optional() {
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut Default::default()),
            TSProperty {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: TSUnion {
                    descriptors: vec![TSPrimitive::String.into(), TSPrimitive::Undefined.into()]
                }
                .into(),
                required: false,
            }
        );
    }
}
