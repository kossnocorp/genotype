use genotype_lang_core_codegen::*;
use genotype_lang_ts_tree::{TSModule, TSRenderContext};

pub struct TsCodegenModule<'a> {
    module: GtlCodegenModuleAcc<'a, TSRenderContext<'a>>,
}

impl<'a> GtlCodegenModule<'a> for TsCodegenModule<'a> {
    type RenderModule = TSModule;

    type RenderContext = TSRenderContext<'a>;

    // fn update<MutateFn: FnOnce(&mut GtlCodegenModuleAcc<'a, TSRenderContext<'a>>) -> ()>(
    //     &mut self,
    //     mutate_fn: MutateFn,
    // ) {
    //     mutate_fn(&mut self.module);
    // }

    // fn get(&self) -> &'a GtlCodegenModuleAcc<'a, TSRenderContext<'a>> {
    //     &self.module
    // }
}
