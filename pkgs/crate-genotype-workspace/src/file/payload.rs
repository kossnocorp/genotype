use genotype_config::GtConfig;
use genotype_parser::GtModuleParse;

pub enum GtwFilePayload {
    Config(Box<GtConfig>),
    Module(Box<GtModuleParse>),
}
