use crate::prelude::internal::*;

mod error;
pub use error::*;

mod visitor;
use visitor::*;

// region: Module type checking

impl GtpModule {
    /// Type checks the resolved project module.
    pub fn type_check(
        self,
        resolved_modules: &IndexMap<GtpModulePath, GtpModuleResolved>,
    ) -> GtpModule {
        let source = self.source().clone();

        let module_resolved = match self {
            GtpModule::Resolved(module_resolved) => module_resolved,

            GtpModule::Error(_, _) => return self,

            _ => {
                return GtpModule::Error(
                    source,
                    GtpModuleError::InvalidModuleState {
                        current_state: self.state_name(),
                        expected_states: "resolved",
                    },
                );
            }
        };

        GtpModuleTypeChecked::type_check(*module_resolved, resolved_modules)
            .map_or_else(|err| GtpModule::Error(source, err), Into::into)
    }
}

// endregion

// region: Type checked module state

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleTypeChecked {
    pub module_resolved: GtpModuleResolved,
}

impl From<GtpModuleTypeChecked> for GtpModule {
    fn from(value: GtpModuleTypeChecked) -> Self {
        GtpModule::TypeChecked(Box::new(value))
    }
}

impl GtpModuleTypeChecked {
    pub fn type_check(
        module_resolved: GtpModuleResolved,
        resolved_modules: &IndexMap<GtpModulePath, GtpModuleResolved>,
    ) -> Result<Self, GtpModuleError> {
        let module = &module_resolved.project_module_parse.module_parse.module;

        let mut visitor = GtpModuleTypeCheckVisitor::new(&module_resolved, resolved_modules);
        module.traverse(&mut visitor);

        let errors = visitor.into_errors();
        if errors.is_empty() {
            Ok(Self { module_resolved })
        } else {
            Err(GtpModuleError::TypeCheck {
                source_code: module_resolved.project_module_parse.source_code.clone(),
                path: module_resolved.project_module_parse.path.clone(),
                errors,
            })
        }
    }
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_branded_primitive_record_key() {
        let resolved = resolved("Path: @string\nMap: { [Path]: string }");
        let modules =
            IndexMap::from_iter([(resolved.project_module_parse.path.clone(), resolved.clone())]);
        let checked = GtpModuleTypeChecked::type_check(resolved, &modules).unwrap();
        let GtDescriptor::Record(record) = &checked
            .module_resolved
            .project_module_parse
            .module_parse
            .module
            .aliases[1]
            .descriptor
        else {
            panic!("expected record")
        };
        assert!(matches!(
            &record.key,
            GtRecordKey::Reference(reference) if reference.identifier.as_str() == "Path"
        ));
    }

    #[test]
    fn rejects_non_primitive_record_key() {
        let resolved = resolved("Key: { value: string }\nMap: { [Key]: string }");
        let modules =
            IndexMap::from_iter([(resolved.project_module_parse.path.clone(), resolved.clone())]);
        assert!(matches!(
            GtpModuleTypeChecked::type_check(resolved, &modules),
            Err(GtpModuleError::TypeCheck {
                errors,
                ..
            }) if matches!(errors.as_slice(), [GtpModuleTypeCheckError::InvalidRecordKey { .. }])
        ));
    }

    #[test]
    fn reports_multiple_invalid_record_keys() {
        let resolved = resolved(
            "ObjectKey: { value: string }\nBooleanKey: boolean\nFirst: { [ObjectKey]: string }\nContainer: { nested: { [BooleanKey]: string } }",
        );
        let modules =
            IndexMap::from_iter([(resolved.project_module_parse.path.clone(), resolved.clone())]);

        let Err(GtpModuleError::TypeCheck { errors, .. }) =
            GtpModuleTypeChecked::type_check(resolved, &modules)
        else {
            panic!("expected type check errors")
        };

        assert_eq!(errors.len(), 2);
        assert!(matches!(
            &errors[0],
            GtpModuleTypeCheckError::InvalidRecordKey { identifier, .. } if identifier == "ObjectKey"
        ));
        assert!(matches!(
            &errors[1],
            GtpModuleTypeCheckError::InvalidRecordKey { identifier, .. } if identifier == "BooleanKey"
        ));
    }

    #[test]
    fn accepts_nested_branded_primitive_record_key() {
        let resolved = resolved(
            "AddressId: @string\nAddress: string\nUser: { addresses: { [AddressId]: Address } }",
        );
        let modules =
            IndexMap::from_iter([(resolved.project_module_parse.path.clone(), resolved.clone())]);
        let checked = GtpModuleTypeChecked::type_check(resolved, &modules).unwrap();
        let GtDescriptor::Object(user) = &checked
            .module_resolved
            .project_module_parse
            .module_parse
            .module
            .aliases[2]
            .descriptor
        else {
            panic!("expected object")
        };
        let GtDescriptor::Record(addresses) = &user.properties[0].descriptor else {
            panic!("expected record")
        };

        assert!(matches!(
            &addresses.key,
            GtRecordKey::Reference(reference) if reference.identifier.as_str() == "AddressId"
        ));
    }

    #[test]
    fn rejects_invalid_type_check_state() {
        let resolved = resolved("Path: @string");
        let modules = IndexMap::new();
        let type_checked = GtpModuleTypeChecked {
            module_resolved: resolved,
        };

        assert!(matches!(
            GtpModule::TypeChecked(Box::new(type_checked)).type_check(&modules),
            GtpModule::Error(
                _,
                GtpModuleError::InvalidModuleState {
                    current_state: "type checked",
                    expected_states: "resolved"
                }
            )
        ));
    }

    fn resolved(source: &str) -> GtpModuleResolved {
        let path = GtpModulePath::new(RelativePathBuf::from("src/module.type"));
        let module_source = GtpModuleSource::from(path.clone());
        let project_module_parse = GtpModuleParse {
            path,
            source: module_source,
            source_code: source.into(),
            module_parse: GtModule::parse("module".into(), source).unwrap(),
        };
        let project_resolve = GtpResolve::resolve(&IndexMap::from_iter([(
            project_module_parse.path.clone(),
            GtpModule::Parsed(Box::new(project_module_parse.clone())),
        )]))
        .unwrap();
        GtpModuleResolve::resolve(&project_resolve, &project_module_parse)
            .map(|resolve| GtpModuleResolved {
                project_module_parse,
                resolve,
            })
            .unwrap()
    }
}
