use genotype_parser::visitor::{GtVisitor, Traverse};
use genotype_parser::*;
use genotype_project::{GtpModule, GtpModuleParse, GtpResolve};
use genotype_project_core::*;
use miette::NamedSource;
use relative_path::RelativePathBuf;
use std::fmt::Debug;
use std::vec;

// TODO: #[deprecated(note = "Use `Gt` factory methods instead")] and get rid of this
pub fn parse_module(source_code: &str) -> GtModule {
    let id = GtModuleId("module".into());

    // TODO: This flow replicates what GtProject::load does. Find a better way
    // to share this code or simplify each step into functions.
    let module_path = GtpModulePath::new(
        RelativePathBuf::from_path("src/module.type").expect("must be correct path"),
    );
    let source_code = NamedSource::new("src/module.type", source_code.into());
    let module_parse = GtModule::parse(id, source_code).expect("source code must be correct");
    let project_parse = GtpModuleParse(module_path, module_parse);
    let modules_parse = vec![project_parse];

    let resolve: GtpResolve = (&modules_parse).try_into().expect("must resolve");
    let modules = modules_parse
        .iter()
        .map(|parse| GtpModule::try_new(&resolve, &modules_parse, parse.clone()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    modules.first().unwrap().module.clone()
}

// TODO: Use #[deprecated(note = "Use `Gt` factory methods instead")] and get rid of this
pub fn parse_get_named<Type>(name: &str, source_code: &str) -> Type
where
    Type: TryFrom<GtDescriptor>,
    Type::Error: Debug,
{
    let module = parse_module(source_code);
    let mut visitor = UnwrapNamedVisitor::new(name);
    module.traverse(&mut visitor);

    let descriptor = visitor.descriptor.expect("named descriptor must exist");
    match Type::try_from(descriptor.clone()) {
        Ok(inner) => inner,
        Err(err) => {
            println!("named descriptor must enclose given type: {err:?}");
            println!("descriptor: {:?}", descriptor);
            panic!("Failed to convert descriptor to type");
        }
    }
}

struct UnwrapNamedVisitor {
    name: String,
    descriptor: Option<GtDescriptor>,
}

impl UnwrapNamedVisitor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            descriptor: None,
        }
    }
}

impl GtVisitor for UnwrapNamedVisitor {
    fn visit_alias(&mut self, alias: &GtAlias) {
        if alias.name.1.as_ref() == self.name.as_str() {
            if self.descriptor.is_some() {
                panic!("multiple descriptors with the same name found");
            }
            self.descriptor = Some(alias.descriptor.clone());
        }
    }
}
