use crate::prelude::internal::*;

pub type GtlModuleTypeImport<'context, Module> = <Module as GtlModule<'context>>::Import;

pub type GtlModuleTypeDependencyIdent<'context, Module> =
    GtlImportTypeDependencyIdent<GtlModuleTypeImport<'context, Module>>;

pub type GtlModuleTypeRenderTypes<'context, Module> = <Module as GtlModule<'context>>::RenderTypes;

pub type GtlModuleTypeRenderContext<'context, Module> =
    <GtlModuleTypeRenderTypes<'context, Module> as GtlRenderTypes<'context>>::Context;

pub trait GtlModule<'context>: GtlRender<'context, Self::RenderTypes> + Debug + Clone {
    type Import: GtlImport;
    type RenderTypes: GtlRenderTypes<'context>;

    fn dependencies(&self) -> IndexSet<GtlModuleTypeDependencyIdent<'context, Self>> {
        let mut dependencies = IndexSet::new();
        for import in self.imports().iter() {
            dependencies.insert(import.dependency().clone());
        }
        dependencies
    }

    fn imports(&self) -> Vec<&Self::Import>;
}
