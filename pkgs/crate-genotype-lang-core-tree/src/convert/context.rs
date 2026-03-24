use crate::*;

pub trait GtlConvertContext {
    // type Import: GtlImport;

    type DependencyIdent: GtlDependencyIdent;

    type DependencyRef: GtlDependencyRef;

    // type Export: GtlExport;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef);

    // fn add_export();
}
