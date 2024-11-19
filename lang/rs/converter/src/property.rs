use genotype_lang_rs_tree::{field::RSField, RSOption};
use genotype_parser::tree::property::GTProperty;
use miette::Result;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

impl RSConvert<RSField> for GTProperty {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSField> {
        let doc = if let Some(doc) = &self.doc {
            Some(doc.convert(context)?)
        } else {
            None
        };

        let name = self.name.convert(context)?;
        context.enter_parent(RSContextParent::Field(name.clone()));

        let descriptor = self.descriptor.convert(context)?;
        let descriptor = if self.required {
            descriptor
        } else {
            RSOption::new(descriptor).into()
        };

        let field = RSField {
            doc,
            attributes: vec![],
            name,
            descriptor,
        };

        context.exit_parent();
        Ok(field)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
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
                required: false,
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSOption::new(RSPrimitive::String.into()).into(),
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut context)
            .unwrap(),
            RSField {
                doc: None,
                attributes: vec![],
                name: "name".into(),
                descriptor: RSOption::new(RSPrimitive::String.into()).into(),
            }
        );
        assert_eq!(context.as_dependencies(), vec![]);
    }
}
