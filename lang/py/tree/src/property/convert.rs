use crate::prelude::internal::*;

impl PYConvert<PYProperty> for GTProperty {
    fn convert(&self, context: &mut PYConvertContext) -> PYProperty {
        PYProperty {
            doc: self.doc.as_ref().and_then(|doc| Some(doc.convert(context))),
            name: self.name.convert(context),
            descriptor: self.descriptor.convert(context),
            required: self.required,
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
            .convert(&mut PYConvertContext::default()),
            PYProperty {
                doc: None,
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: false,
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            Default::default(),
            PyConfigLang::new(PYVersion::Legacy),
            Default::default(),
        );
        assert_eq!(
            GTProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut context),
            PYProperty {
                doc: None,
                name: "name".into(),
                descriptor: PYPrimitive::String.into(),
                required: false,
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependencyIdent::Typing, "Optional".into())]
        );
    }
}
