use genotype_lang_core_tree::indent::GTIndent;

pub fn rs_indent() -> GTIndent<'static> {
    GTIndent::start("    ")
}
