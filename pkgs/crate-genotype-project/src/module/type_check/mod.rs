use crate::prelude::internal::*;

mod error;
pub use error::*;

mod record;
pub use record::*;

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
    pub record_key_resolves: IndexMap<GtReferenceId, GtpModuleTypeCheckRecordKeyResolve>,
}

impl GtpModuleTypeChecked {
    pub fn build_info(&self) -> GtpBuildInfoSrcModule {
        let resolved = &self.module_resolved;
        let project_module_parse = &resolved.project_module_parse;

        let id = project_module_parse.module_parse.module.id.clone();
        let hash = project_module_parse.source_code.hash.clone();
        let deps = resolved.resolve.deps.iter().cloned().collect::<Vec<_>>();

        GtpBuildInfoSrcModule { id, hash, deps }
    }
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

        let (errors, record_key_resolves) = visitor.into_result();
        if errors.is_empty() {
            Ok(Self {
                module_resolved,
                record_key_resolves,
            })
        } else {
            Err(GtpModuleError::TypeCheck {
                source_code: module_resolved
                    .project_module_parse
                    .source_code
                    .content
                    .clone(),
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
    fn accepts_boolean_record_key_alias() {
        let resolved = resolved(
            "BooleanKey: boolean\nBooleanAlias: BooleanKey\nBrandedBooleanKey: @boolean\nFirst: { [BooleanKey]: string }\nSecond: { [BooleanAlias]: string }\nThird: { [BrandedBooleanKey]: string }",
        );
        let modules =
            IndexMap::from_iter([(resolved.project_module_parse.path.clone(), resolved.clone())]);
        let checked = GtpModuleTypeChecked::type_check(resolved, &modules).unwrap();

        assert_eq!(checked.record_key_resolves.len(), 3);
        assert!(
            checked
                .record_key_resolves
                .values()
                .all(|resolve| resolve.primitive == GtPrimitiveKind::Boolean)
        );
        assert_eq!(
            checked
                .record_key_resolves
                .values()
                .filter(|resolve| resolve.branded)
                .count(),
            1
        );
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
            record_key_resolves: IndexMap::new(),
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
            source_code: GtpSourceCode {
                hash: GtpSourceCodeHash::new("hash"),
                content: source.into(),
            },
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
