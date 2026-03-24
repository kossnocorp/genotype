use genotype_parser::visitor::{GTVisitor, Traverse};
use genotype_parser::*;
use genotype_path::*;
use genotype_project::{GTPResolve, GTProjectModuleParse, GtProjectModule};
use miette::NamedSource;
use relative_path::RelativePathBuf;
use std::fmt::Debug;
use std::vec;

#[deprecated(note = "Use `Gt` factory methods instead")]
pub fn parse_module(source_code: &str) -> GTModule {
    let id = GTModuleId("module".into());

    // TODO: This flow replicates what GtProject::load does. Find a better way
    // to share this code or simplify each step into functions.
    let module_path = GtModulePath::new(
        RelativePathBuf::from_path("src/module.type").expect("must be correct path"),
    );
    let source_code = NamedSource::new("src/module.type", source_code.into());
    let module_parse = GTModule::parse(id, source_code).expect("source code must be correct");
    let project_parse = GTProjectModuleParse(module_path, module_parse);
    let modules_parse = vec![project_parse];

    let resolve: GTPResolve = (&modules_parse).try_into().expect("must resolve");
    let modules = modules_parse
        .iter()
        .map(|parse| GtProjectModule::try_new(&resolve, &modules_parse, parse.clone()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    modules.first().unwrap().module.clone()
}

#[deprecated(note = "Use `Gt` factory methods instead")]
pub fn parse_get_named<Type>(name: &str, source_code: &str) -> Type
where
    Type: TryFrom<GTDescriptor>,
    Type::Error: Debug,
{
    let mut module = parse_module(source_code);
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
    descriptor: Option<GTDescriptor>,
}

impl UnwrapNamedVisitor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            descriptor: None,
        }
    }
}

impl GTVisitor for UnwrapNamedVisitor {
    fn visit_alias(&mut self, alias: &mut GTAlias) {
        if alias.name.1.as_ref() == self.name.as_str() {
            if let Some(_) = self.descriptor {
                panic!("multiple descriptors with the same name found");
            }
            self.descriptor = Some(alias.descriptor.clone());
        }
    }
}
