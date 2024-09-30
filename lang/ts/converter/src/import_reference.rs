use genotype_lang_ts_tree::{
    definition::TSDefinition, import_glob_alias::TSImportGlobAlias,
    import_reference::TSImportReference,
};
use genotype_parser::tree::import_reference::GTImportReference;

use crate::convert::TSConvert;

impl TSConvert<TSImportReference> for GTImportReference {
    fn convert<HoistFn>(&self, hoist: &HoistFn) -> TSImportReference
    where
        HoistFn: Fn(TSDefinition),
    {
        match self {
            Self::Glob => TSImportReference::Glob(TSImportGlobAlias::Unresolved),

            Self::Names(names) => TSImportReference::Named(
                names
                    .iter()
                    .map(|name| name.convert(hoist))
                    .collect::<Vec<_>>(),
            ),

            Self::Name(name) => TSImportReference::Named(vec![name.convert(hoist)]),
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::{import_name::TSImportName, name::TSName};
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{import_name::GTImportName, name::GTName};

    #[test]
    fn test_convert_glob() {
        assert_eq!(
            GTImportReference::Glob.convert(&|_| {}),
            TSImportReference::Glob(TSImportGlobAlias::Unresolved)
        );
    }

    #[test]
    fn test_convert_names() {
        assert_eq!(
            GTImportReference::Names(vec![
                GTImportName::Name(GTName("Name".into())),
                GTImportName::Alias(GTName("Name".into()), GTName("Alias".into()))
            ])
            .convert(&|_| {}),
            TSImportReference::Named(vec![
                TSImportName::Name(TSName("Name".into())),
                TSImportName::Alias(TSName("Name".into()), TSName("Alias".into()))
            ])
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImportReference::Name(GTName("Name".into())).convert(&|_| {}),
            TSImportReference::Named(vec![TSImportName::Name(TSName("Name".into()))])
        );
    }
}
