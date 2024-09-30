use genotype_lang_core_tree::indent::GTIndent;

pub fn ts_indent() -> GTIndent<'static> {
    GTIndent::start("  ")
}
