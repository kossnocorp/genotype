use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSEnumVariant;

impl RSRender for RSEnumVariant {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::RSLangConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    #[ignore = "WIP"]
    fn test_render_descriptor() {
        assert_eq!(
            RSEnumVariant {
                doc: None,
                attributes: vec![],
                name: "Variant".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::Boolean).into(),
            }
            .render(&rs_indent(), &Default::default()),
            "Variant(bool)"
        );
    }

    // #[test]
    // fn test_render_legacy() {
    //     assert_eq!(
    //         RSEnum {
    //             descriptors: vec![
    //                 RSDescriptor::Primitive(RSPrimitive::String),
    //                 RSDescriptor::Primitive(RSPrimitive::Int),
    //             ],
    //             discriminator: None
    //         }
    //         .render(&rs_indent(), &RSLangConfig::new(RSVersion::Legacy)),
    //         "Union[String, isize]"
    //     );
    // }

    // #[test]
    // fn test_render_discriminator() {
    //     assert_eq!(
    //         RSEnum {
    //             descriptors: vec![
    //                 RSDescriptor::Primitive(RSPrimitive::String),
    //                 RSDescriptor::Primitive(RSPrimitive::Int),
    //             ],
    //             discriminator: Some("type".into())
    //         }
    //         .render(&rs_indent(), &Default::default()),
    //         r#"Annotated[String | isize, Field(json_schema_extra={'descriminator': 'type'})]"#
    //     );
    // }

    // #[test]
    // fn test_render_discriminator_legacy() {
    //     assert_eq!(
    //         RSEnum {
    //             descriptors: vec![
    //                 RSDescriptor::Primitive(RSPrimitive::String),
    //                 RSDescriptor::Primitive(RSPrimitive::Int),
    //             ],
    //             discriminator: Some("type".into())
    //         }
    //         .render(&rs_indent(), &RSLangConfig::new(RSVersion::Legacy)),
    //         r#"Annotated[Union[String, isize], Field(json_schema_extra={'descriminator': 'type'})]"#
    //     );
    // }
}
