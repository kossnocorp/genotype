use crate::prelude::internal::*;

impl PyConvert<PyImport> for GtImport {
    fn convert(&self, context: &mut PyConvertContext) -> PyImport {
        let reference = match &self.reference {
            GtImportReference::Glob(_) => PyImportReference::Default(Some(module_name(&self.path))),

            GtImportReference::Names(_, names) => PyImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(context))
                    .collect::<Vec<_>>(),
            ),

            GtImportReference::Name(_, name) => {
                PyImportReference::Named(vec![PyImportName::Name(name.convert(context))])
            }
        };

        let path = self.path.convert(context);

        PyImport {
            reference,
            dependency: PyDependencyIdent::Path(path),
        }
    }
}

fn module_name(path: &GtPath) -> PyIdentifier {
    let str = path.source_str();
    str.split("/").last().unwrap_or(str).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_glob() {
        let mut resolve = PyConvertResolve::default();
        resolve.globs.insert(
            GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
            "module".into(),
        );
        let mut context = PyConvertContext::default();
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GtImportReference::Glob((0, 0).into())
            }
            .convert(&mut context),
            @r#"
        PyImport(
          dependency: Path(PyPath(".path.to.module")),
          reference: Default(Some(PyIdentifier("module"))),
        )
        "#
        );
    }

    #[test]
    fn test_convert_names() {
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GtImportReference::Names(
                    (0, 0).into(),
                    vec![
                        GtImportName::Name(
                            (0, 0).into(),
                            GtIdentifier::new((0, 0).into(), "Name".into())
                        ),
                        GtImportName::Alias(
                            (0, 0).into(),
                            GtIdentifier::new((0, 0).into(), "Name".into()),
                            GtIdentifier::new((0, 0).into(), "Alias".into())
                        )
                    ]
                )
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        PyImport(
          dependency: Path(PyPath(".path.to.module")),
          reference: Named([
            Name(PyIdentifier("Name")),
            Alias(PyIdentifier("Name"), PyIdentifier("Alias")),
          ]),
        )
        "#
        );
    }

    #[test]
    fn test_convert_name() {
        assert_ron_snapshot!(
            GtImport {
                span: (0, 0).into(),
                path: GtPath::parse((0, 0).into(), "./path/to/module").unwrap(),
                reference: GtIdentifier::new((0, 0).into(), "Name".into()).into()
            }
            .convert(&mut PyConvertContext::default()),
            @r#"
        PyImport(
          dependency: Path(PyPath(".path.to.module")),
          reference: Named([
            Name(PyIdentifier("Name")),
          ]),
        )
        "#
        );
    }
}
