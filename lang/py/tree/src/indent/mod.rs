use genotype_lang_core_tree::indent::GTIndent;

pub fn py_indent() -> GTIndent<'static> {
    GTIndent::start("    ")
}
