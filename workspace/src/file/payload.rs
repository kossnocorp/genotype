use genotype_config::GTConfig;
use genotype_parser::GTModuleParse;

pub enum GTWFilePayload {
    Config(GTConfig),
    Module(GTModuleParse),
}
