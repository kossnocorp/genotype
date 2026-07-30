use crate::prelude::internal::*;

pub struct GtpModuleTypeCheckVisitor<'a> {
    module_resolved: &'a GtpModuleResolved,
    modules: &'a IndexMap<GtpModulePath, GtpModuleResolved>,
    errors: Vec<GtpModuleTypeCheckError>,
}

impl<'a> GtpModuleTypeCheckVisitor<'a> {
    pub fn new(
        module_resolved: &'a GtpModuleResolved,
        modules: &'a IndexMap<GtpModulePath, GtpModuleResolved>,
    ) -> Self {
        Self {
            module_resolved,
            modules,
            errors: vec![],
        }
    }

    pub fn into_errors(self) -> Vec<GtpModuleTypeCheckError> {
        self.errors
    }

    fn validate_record_key(&self, reference: &GtReference) -> Result<(), &'static str> {
        let mut definition_id = self
            .module_resolved
            .resolve
            .reference_definition_ids
            .get(&reference.id)
            .ok_or("record key reference cannot be resolved")?
            .clone();

        let mut visited = IndexSet::new();

        loop {
            if !visited.insert(definition_id.clone()) {
                return Err("record key alias is cyclic");
            }

            let module = self
                .modules
                .values()
                .find(|module| {
                    module.project_module_parse.module_parse.module.id == definition_id.0
                })
                .ok_or("package record key aliases are not supported")?;

            let alias = module
                .project_module_parse
                .module_parse
                .module
                .aliases
                .iter()
                .find(|alias| alias.id == definition_id)
                .ok_or("record key alias definition cannot be found")?;

            if !alias.generics.is_empty() {
                return Err("generic aliases cannot be record keys");
            }

            match &alias.descriptor {
                GtDescriptor::Primitive(primitive) => {
                    return Self::validate_primitive_record_key(&primitive.kind);
                }

                GtDescriptor::Branded(branded) => {
                    return Self::validate_primitive_record_key(&branded.primitive.kind);
                }

                GtDescriptor::Reference(next) if next.arguments.is_empty() => {
                    definition_id = module
                        .resolve
                        .reference_definition_ids
                        .get(&next.id)
                        .ok_or("record key alias reference cannot be resolved")?
                        .clone();
                }

                _ => return Err("record key alias must resolve to a supported primitive"),
            }
        }
    }

    fn validate_primitive_record_key(kind: &GtPrimitiveKind) -> Result<(), &'static str> {
        match kind {
            GtPrimitiveKind::Boolean => Err("boolean aliases cannot be record keys"),

            _ => Ok(()),
        }
    }
}

impl GtVisitor for GtpModuleTypeCheckVisitor<'_> {
    fn visit_record(&mut self, record: &GtRecord) {
        let GtRecordKey::Reference(reference) = &record.key else {
            return;
        };

        if let Err(reason) = self.validate_record_key(reference) {
            self.errors.push(GtpModuleTypeCheckError::InvalidRecordKey {
                span: reference.span,
                identifier: reference.identifier.as_string(),
                reason,
            });
        }
    }
}
