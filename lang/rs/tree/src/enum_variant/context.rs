use genotype_lang_rs_config::RSVersion;

use crate::*;

use super::RSEnumVariant;

impl RSContextResolve for RSEnumVariant {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        // if context.is_version(RSVersion::Legacy) {
        //     context.import(RSDependency::Typing, "Union".into());
        // }

        // if self.discriminator.is_some() {
        //     if context.is_version(RSVersion::Legacy) {
        //         context.import(RSDependency::TypingExtensions, "Annotated".into());
        //     } else {
        //         context.import(RSDependency::Typing, "Annotated".into());
        //     }

        //     context.import(RSDependency::Rsdantic, "Field".into());
        // }

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use genotype_lang_rs_config::RSVersion;
    use mock::RSContextMock;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn test_resolve() {
    //     let mut context = RSContextMock::default();
    //     let union = RSEnum {
    //         descriptors: vec![RSPrimitive::String.into()],
    //         discriminator: None,
    //     };
    //     union.resolve(&mut context);
    //     assert_eq!(context.as_imports(), vec![]);
    // }

    // #[test]
    // fn test_resolve_legacy() {
    //     let mut context = RSContextMock::new(RSVersion::Legacy);
    //     let union = RSEnum {
    //         descriptors: vec![RSPrimitive::String.into()],
    //         discriminator: None,
    //     };
    //     union.resolve(&mut context);
    //     assert_eq!(
    //         context.as_imports(),
    //         vec![(RSDependency::Typing, "Union".into())]
    //     );
    // }

    // #[test]
    // fn test_resolve_discriminator() {
    //     let mut context = RSContextMock::default();
    //     let union = RSEnum {
    //         descriptors: vec![RSPrimitive::String.into()],
    //         discriminator: Some("type".into()),
    //     };
    //     union.resolve(&mut context);
    //     assert_eq!(
    //         context.as_imports(),
    //         vec![
    //             (RSDependency::Typing, "Annotated".into()),
    //             (RSDependency::Rsdantic, "Field".into())
    //         ]
    //     );
    // }

    // #[test]
    // fn test_resolve_discriminator_legacy() {
    //     let mut context = RSContextMock::new(RSVersion::Legacy);
    //     let union = RSEnum {
    //         descriptors: vec![RSPrimitive::String.into()],
    //         discriminator: Some("type".into()),
    //     };
    //     union.resolve(&mut context);
    //     assert_eq!(
    //         context.as_imports(),
    //         vec![
    //             (RSDependency::Typing, "Union".into()),
    //             (RSDependency::TypingExtensions, "Annotated".into()),
    //             (RSDependency::Rsdantic, "Field".into())
    //         ]
    //     );
    // }
}
