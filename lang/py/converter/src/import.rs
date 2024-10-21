use genotype_lang_py_tree::*;
use genotype_parser::*;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYImport> for GTImport {
    fn convert(&self, context: &mut PYConvertContext) -> PYImport {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => PYImportReference::Default(Some(module_name(&self.path))),

            GTImportReference::Names(_, names) => PYImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Vec<_>>(),
            ),

            GTImportReference::Name(_, name) => {
                PYImportReference::Named(vec![PYImportName::Name(name.convert(context))])
            }
        };

        let path = self.path.convert(context);

        PYImport {
            path: path.clone(),
            reference,
            dependency: PYDependency::Local(path),
        }
    }
}

fn module_name(path: &GTPath) -> PYIdentifier {
    let str = path.as_str();
    str.split("/").last().unwrap_or(str).into()
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::PYConvertResolve;

    use super::*;

    #[test]
    fn test_convert_glob() {
        let mut resolve = PYConvertResolve::default();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTImportReference::Glob((0, 0).into())
            }
            .convert(&mut context),
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Default(Some("module".into())),
                dependency: PYDependency::Local(".path.to.module".into())
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
            .convert(&mut PYConvertContext::default()),
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Named(vec![
                    PYImportName::Name("Name".into()),
                    PYImportName::Alias("Name".into(), "Alias".into())
                ]),
                dependency: PYDependency::Local(".path.to.module".into())
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
            .convert(&mut PYConvertContext::default()),
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Named(vec![PYImportName::Name("Name".into())]),
                dependency: PYDependency::Local(".path.to.module".into())
            }
        );
    }
}
