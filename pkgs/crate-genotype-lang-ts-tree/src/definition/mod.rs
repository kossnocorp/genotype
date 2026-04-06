use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsDefinition {
    Alias(#[visit] TsAlias),
    Interface(#[visit] TsInterface),
    Branded(#[visit] TsBranded),
    Embed(#[visit] TsEmbedDefinition),
}

impl TsDefinition {
    pub fn name(&self) -> TsIdentifier {
        match self {
            TsDefinition::Alias(alias) => alias.name.clone(),
            TsDefinition::Interface(interface) => interface.name.clone(),
            TsDefinition::Branded(branded) => branded.name.clone(),
            TsDefinition::Embed(embed) => embed.name.clone(),
        }
    }

    pub fn scan_dependencies(&self) -> TsDefinitionDependenciesScanVisitor {
        let mut visitor = TsDefinitionDependenciesScanVisitor::new();
        self.traverse(&mut visitor);
        visitor
    }
}

pub struct TsDefinitionDependenciesScanVisitor {
    pub dependencies: HashSet<TsIdentifier>,
}

impl TsDefinitionDependenciesScanVisitor {
    pub fn new() -> TsDefinitionDependenciesScanVisitor {
        TsDefinitionDependenciesScanVisitor {
            dependencies: HashSet::new(),
        }
    }
}

impl TsVisitor for TsDefinitionDependenciesScanVisitor {
    fn visit_extension(&mut self, node: &TsExtension) {
        self.dependencies.insert(node.reference.identifier.clone());
    }

    fn visit_inline_import(&mut self, node: &TsInlineImport) {
        self.dependencies.insert(node.name.clone());
    }

    fn visit_reference(&mut self, node: &TsReference) {
        self.dependencies.insert(node.identifier.clone());
    }
}

impl GtlDefinition for TsDefinition {}

impl From<TsBranded> for TsDefinition {
    fn from(branded: TsBranded) -> Self {
        TsDefinition::Branded(branded)
    }
}

impl From<TsAlias> for TsDefinition {
    fn from(alias: TsAlias) -> Self {
        TsDefinition::Alias(alias)
    }
}

impl From<TsEmbedDefinition> for TsDefinition {
    fn from(embed: TsEmbedDefinition) -> Self {
        TsDefinition::Embed(embed)
    }
}

impl From<TsInterface> for TsDefinition {
    fn from(interface: TsInterface) -> Self {
        TsDefinition::Interface(interface)
    }
}
