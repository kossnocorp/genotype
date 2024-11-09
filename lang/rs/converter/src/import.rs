use genotype_lang_rs_tree::*;
use genotype_parser::*;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSImport> for GTImport {
    fn convert(&self, context: &mut RSConvertContext) -> RSImport {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => RSImportReference::Default(Some(module_name(&self.path))),

            GTImportReference::Names(_, names) => RSImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Vec<_>>(),
            ),

            GTImportReference::Name(_, name) => {
                RSImportReference::Named(vec![RSImportName::Name(name.convert(context))])
            }
        };

        let path = self.path.convert(context);

        RSImport {
            path: path.clone(),
            reference,
            dependency: RSDependency::Local(path),
        }
    }
}

fn module_name(path: &GTPath) -> RSIdentifier {
    let str = path.as_str();
    str.split("/").last().unwrap_or(str).into()
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
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTImportReference::Glob((0, 0).into())
            }
            .convert(&mut context),
            RSImport {
                path: "self::path::to::module".into(),
                reference: RSImportReference::Default(Some("module".into())),
                dependency: RSDependency::Local("self::path::to::module".into())
            }
        );
    }

    #[test]
    fn test_convert_names() {
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
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
            .convert(&mut RSConvertContext::default()),
            RSImport {
                path: "self::path::to::module".into(),
                reference: RSImportReference::Named(vec![
                    RSImportName::Name("Name".into()),
                    RSImportName::Alias("Name".into(), "Alias".into())
                ]),
                dependency: RSDependency::Local("self::path::to::module".into())
            }
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut RSConvertContext::default()),
            RSImport {
                path: "self::path::to::module".into(),
                reference: RSImportReference::Named(vec![RSImportName::Name("Name".into())]),
                dependency: RSDependency::Local("self::path::to::module".into())
            }
        );
    }
}
