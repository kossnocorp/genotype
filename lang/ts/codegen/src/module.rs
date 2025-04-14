use genotype_lang_core_codegen::*;
use genotype_lang_ts_tree::*;

pub struct TsCodegenModule<'a> {
    module: GtlCodegenModuleAcc<'a, TSRenderState, TSRenderContext>,
}

impl<'a> GtlCodegenModule<'a> for TsCodegenModule<'a> {
    type RenderModule = TSModule;

    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    // fn update<MutateFn: FnOnce(&mut GtlCodegenModuleAcc<'a, TSRenderContext>) -> ()>(
    //     &mut self,
    //     mutate_fn: MutateFn,
    // ) {
    //     mutate_fn(&mut self.module);
    // }

    // fn get(&self) -> &'a GtlCodegenModuleAcc<'a, TSRenderContext> {
    //     &self.module
    // }
}
