use genotype_config::GtConfig;
use genotype_parser::GTModuleParse;

pub enum GTWFilePayload {
    Config(GtConfig),
    Module(GTModuleParse),
}
