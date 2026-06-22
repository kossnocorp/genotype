use crate::prelude::internal::*;

mod convert;

mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsModule {
    pub id: GtModuleId,
    #[visit]
    pub doc: Option<RsDoc>,
    #[visit]
    pub imports: Vec<RsUse>,
    #[visit]
    pub definitions: Vec<RsDefinition>,
}

impl GtlModule<'_> for RsModule {
    type Import = RsUse;
    type RenderTypes = RsRenderTypes;

    fn imports(&self) -> Vec<&RsUse> {
        self.imports.iter().collect()
    }
}
