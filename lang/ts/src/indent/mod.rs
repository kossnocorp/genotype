use genotype_lang_core::indent::GTIndent;

pub fn ts_indent() -> GTIndent<'static> {
    GTIndent::start("  ")
}
