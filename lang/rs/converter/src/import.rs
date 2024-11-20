use genotype_lang_rs_tree::*;
use genotype_parser::*;
use miette::Result;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSUse> for GTImport {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSUse> {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => RSUseReference::Module,

            GTImportReference::Names(_, names) => RSUseReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Result<Vec<_>>>()?,
            ),

            GTImportReference::Name(_, name) => {
                RSUseReference::Named(vec![RSUseName::Name(name.convert(context)?)])
            }
        };

        let path = self.path.convert(context)?;

        Ok(RSUse {
            reference,
            dependency: RSDependency::Local(path),
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::RSConvertResolve;

    use super::*;

    #[test]
    fn test_convert_glob() {
        let mut resolve = RSConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GTImportReference::Glob((0, 0).into())
            }
            .convert(&mut context)
            .unwrap(),
            RSUse {
                reference: RSUseReference::Module,
                dependency: RSDependency::Local(RSPath(
                    "module/path".into(),
                    "super::path::to::module".into()
                ))
            }
        );
    }

    #[test]
    fn test_convert_names() {
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GTImportReference::Names(
                    (0, 0).into(),
                    vec![
                        GTImportName::Name(
                            (0, 0).into(),
                            GTIdentifier::new((0, 0).into(), "Name".into())
                        ),
                        GTImportName::Alias(
                            (0, 0).into(),
                            GTIdentifier::new((0, 0).into(), "Name".into()),
                            GTIdentifier::new((0, 0).into(), "Alias".into())
                        )
                    ]
                )
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSUse {
                reference: RSUseReference::Named(vec![
                    RSUseName::Name("Name".into()),
                    RSUseName::Alias("Name".into(), "Alias".into())
                ]),
                dependency: RSDependency::Local(RSPath(
                    "module/path".into(),
                    "super::path::to::module".into()
                ))
            }
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::new(
                    (0, 0).into(),
                    GTPathModuleId::Resolved("module/path".into()),
                    "./path/to/module".into()
                ),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSUse {
                reference: RSUseReference::Named(vec![RSUseName::Name("Name".into())]),
                dependency: RSDependency::Local(RSPath(
                    "module/path".into(),
                    "super::path::to::module".into()
                ))
            }
        );
    }
}
