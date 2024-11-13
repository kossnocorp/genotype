use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSEnum;

impl RSRender for RSEnum {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        // let content = self
        //     .descriptors
        //     .iter()
        //     .map(|d| d.render(indent, config))
        //     .collect::<Vec<String>>()
        //     .join(if let RSVersion::Legacy = config.version {
        //         ", "
        //     } else {
        //         " | "
        //     });

        // let union = if let RSVersion::Legacy = config.version {
        //     format!("Union[{}]", content)
        // } else {
        //     content
        // };

        // if let Some(discriminator) = &self.discriminator {
        //     format!(
        //         r#"Annotated[{}, Field(json_schema_extra={{'descriminator': '{}'}})]"#,
        //         union, discriminator
        //     )
        // } else {
        //     union
        // }
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::RSLangConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    // #[test]
    // fn test_render_union() {
    //     assert_eq!(
    //         RSEnum {
    //             descriptors: vec![
    //                 RSDescriptor::Primitive(RSPrimitive::String),
    //                 RSDescriptor::Primitive(RSPrimitive::Int),
    //             ],
    //             discriminator: None
    //         }
    //         .render(&rs_indent(), &Default::default()),
    //         "String | isize"
    //     );
    // }

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
