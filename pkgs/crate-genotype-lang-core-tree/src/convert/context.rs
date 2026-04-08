use crate::prelude::internal::*;

pub trait GtlConvertContext {
    // region: Imports

    type Import: GtlImport;

    fn imports(&self) -> &Vec<Self::Import>;

    fn imports_mut(&mut self) -> &mut Vec<Self::Import>;

    fn push_import(&mut self, import: Self::Import) {
        self.imports_mut().push(import);
    }

    fn drain_imports(&mut self) -> Vec<Self::Import> {
        let drained_imports: Vec<_> = self.imports_mut().drain(..).collect();
        let mut imports: Vec<Self::Import> = vec![];

        for import in drained_imports {
            // Already imported, skip
            if imports.iter().any(|i| {
                i.dependency() == import.dependency() && i.reference() == import.reference()
            }) {
                continue;
            }

            // Try to find an existing named import and merge with it
            if let Some(import_ref_names) = import.ref_names() {
                let same_dep_import = imports
                    .iter_mut()
                    .find(|i| i.dependency() == import.dependency());

                if let Some(same_dep_import) = same_dep_import
                    && let Some(same_dep_ref_names) = same_dep_import.ref_names_mut() {
                        for name in import_ref_names {
                            if !same_dep_ref_names.contains(name) {
                                same_dep_ref_names.push(name.clone());
                            }
                        }
                        continue;
                    }
            }

            imports.push(import);
        }

        imports.into_iter().collect()
    }

    // endregion

    // TODO: Extract lang code:
    //
    // 1. Definitions + hoisting
    // 2. Docs
    // 2. `resolve_path` and related
}
