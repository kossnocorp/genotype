use genotype_lang_py_tree::{
    definition::PYDefinition, import::PYImport, PYImportName, PYImportReference,
};
use genotype_parser::tree::{import::GTImport, GTImportReference};

use crate::{convert::PYConvert, resolve::PYConvertResolve};

impl PYConvert<PYImport> for GTImport {
    fn convert<HoistFn>(&self, resolve: &PYConvertResolve, hoist: &HoistFn) -> PYImport
    where
        HoistFn: Fn(PYDefinition),
    {
        let reference = match &self.reference {
            GTImportReference::Glob(_) => {
                // [TODO]
                PYImportReference::Glob
            }

            GTImportReference::Names(_, names) => PYImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(resolve, hoist))
                    .collect::<Vec<_>>(),
            ),

            GTImportReference::Name(_, name) => {
                PYImportReference::Named(vec![PYImportName::Name(name.convert(resolve, hoist))])
            }
        };

        PYImport {
            path: self.path.convert(resolve, hoist),
            reference,
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::*;

    #[test]
    fn test_convert_glob() {
        let mut resolve = PYConvertResolve::new();
        resolve.globs.insert(
            GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        assert_eq!(
            GTImport {
                span: (0, 0).into(),
                path: GTPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GTImportReference::Glob((0, 0).into())
            }
            .convert(&resolve, &|_| {}),
            PYImport {
                path: "./path/to/module.ts".into(),
                reference: PYImportReference::Glob
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
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYImport {
                path: "./path/to/module.ts".into(),
                reference: PYImportReference::Named(vec![
                    PYImportName::Name("Name".into()),
                    PYImportName::Alias("Name".into(), "Alias".into())
                ])
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
            .convert(&PYConvertResolve::new(), &|_| {}),
            PYImport {
                path: "./path/to/module.ts".into(),
                reference: PYImportReference::Named(vec![PYImportName::Name("Name".into())])
            }
        );
    }
}
