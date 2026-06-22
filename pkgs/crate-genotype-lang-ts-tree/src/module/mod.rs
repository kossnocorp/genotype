use crate::prelude::internal::*;

mod convert;

mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsModule {
    #[visit]
    pub doc: Option<TsDoc>,
    #[visit]
    pub imports: Vec<TsImport>,
    #[visit]
    pub definitions: Vec<TsDefinition>,
}

impl GtlModule<'_> for TsModule {
    type Import = TsImport;
    type RenderTypes = TsRenderTypes;

    fn imports(&self) -> Vec<&TsImport> {
        self.imports.iter().collect()
    }
}
