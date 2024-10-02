use genotype_lang_ts_tree::{
    definition::TSDefinition, import_glob_alias::TSImportGlobAlias, import_name::TSImportName,
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

            Self::Name(name) => {
                TSImportReference::Named(vec![TSImportName::Name(name.convert(hoist))])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_ts_tree::{import_name::TSImportName, reference::TSReference};
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::{import_name::GTImportName, reference::GTReference};

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
                GTImportName::Name("Name".into()),
                GTImportName::Alias("Name".into(), "Alias".into())
            ])
            .convert(&|_| {}),
            TSImportReference::Named(vec![
                TSImportName::Name("Name".into()),
                TSImportName::Alias("Name".into(), "Alias".into())
            ])
        );
    }

    #[test]
    fn test_convert_name() {
        assert_eq!(
            GTImportReference::Name("Name".into()).convert(&|_| {}),
            TSImportReference::Named(vec![TSImportName::Name("Name".into())])
        );
    }
}
