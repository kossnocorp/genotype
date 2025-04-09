use genotype_lang_core_tree::indent::GTIndent;
use genotype_parser::{GTAlias, GTDescriptor};
use miette::Result;

pub trait GtlCodegen {
    fn indent() -> GTIndent<'static> {
        GTIndent::start("  ")
    }

    fn render_descriptor(descriptor: &GTDescriptor) -> Result<(String, Option<String>)>;

    fn render_alias(alias: &GTAlias) -> Result<String>;
}
