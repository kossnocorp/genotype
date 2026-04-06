use crate::prelude::internal::*;
use std::sync::LazyLock;

pub use genotype_test::*;

pub fn convert_node<GtNode: TsConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_node_with<GtNode: TsConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut TsConvertContext,
) -> Node {
    gt_node.convert(context)
}

pub fn render_node_with<'a, TsNode>(node: TsNode, context: &mut TsRenderContext<'a>) -> String
where
    TsNode: GtlRender<'a, RenderState = TsRenderState, RenderContext = TsRenderContext<'a>>,
{
    node.render(Default::default(), context).unwrap()
}

pub fn render_node<TsNode>(node: TsNode) -> String
where
    TsNode:
        GtlRender<'static, RenderState = TsRenderState, RenderContext = TsRenderContext<'static>>,
{
    let mut context = Tst::render_context();
    render_node_with(node, &mut context)
}

static TEST_TS_DEFAULT_CONFIG_LANG: LazyLock<TsConfigLang> = LazyLock::new(TsConfigLang::default);

static TEST_TS_ZOD_CONFIG_LANG: LazyLock<TsConfigLang> = LazyLock::new(|| TsConfigLang {
    mode: TsMode::Zod,
    ..Default::default()
});

static TEST_TS_ZOD_CONFIG: LazyLock<TsConfig> = LazyLock::new(|| TsConfig {
    lang: TEST_TS_ZOD_CONFIG_LANG.clone(),
    ..Default::default()
});

pub struct Tst {}

impl Tst {
    pub fn convert_context() -> TsConvertContext {
        Default::default()
    }

    pub fn convert_context_zod() -> TsConvertContext {
        TsConvertContext::new(Default::default(), &TEST_TS_ZOD_CONFIG)
    }

    pub fn render_context() -> TsRenderContext<'static> {
        TsRenderContext {
            config: &TEST_TS_DEFAULT_CONFIG_LANG,
            ..Default::default()
        }
    }

    pub fn render_context_zod() -> TsRenderContext<'static> {
        TsRenderContext {
            config: &TEST_TS_ZOD_CONFIG_LANG,
            ..Default::default()
        }
    }

    pub fn doc(doc: &str) -> TsDoc {
        TsDoc(doc.into())
    }

    pub fn some_doc(doc: &str) -> Option<TsDoc> {
        Some(Self::doc(doc))
    }

    pub fn identifier(name: &str) -> TsIdentifier {
        name.into()
    }

    pub fn key(name: &str) -> TsKey {
        name.into()
    }

    pub fn path(path: &str) -> TsPath {
        path.into()
    }

    pub fn primitive_string() -> TsPrimitive {
        TsPrimitive::String
    }

    pub fn primitive_number() -> TsPrimitive {
        TsPrimitive::Number
    }

    pub fn primitive_boolean() -> TsPrimitive {
        TsPrimitive::Boolean
    }

    pub fn primitive_bigint() -> TsPrimitive {
        TsPrimitive::BigInt
    }

    pub fn primitive_null() -> TsPrimitive {
        TsPrimitive::Null
    }

    pub fn primitive_undefined() -> TsPrimitive {
        TsPrimitive::Undefined
    }

    pub fn literal_null() -> TsLiteral {
        TsLiteral::Null
    }

    pub fn literal_boolean(value: bool) -> TsLiteral {
        TsLiteral::Boolean(value)
    }

    pub fn literal_integer(value: i64) -> TsLiteral {
        TsLiteral::Integer(value)
    }

    pub fn literal_float(value: f64) -> TsLiteral {
        TsLiteral::Float(value)
    }

    pub fn literal_string(value: &str) -> TsLiteral {
        TsLiteral::String(value.into())
    }

    pub fn reference(name: &str) -> TsReference {
        TsReference::new(Self::identifier(name), TsReferenceRel::Regular)
    }

    pub fn reference_forward(name: &str) -> TsReference {
        TsReference::new(Self::identifier(name), TsReferenceRel::Forward)
    }

    pub fn reference_self_recursive(name: &str) -> TsReference {
        TsReference::new(Self::identifier(name), TsReferenceRel::SelfRecursive)
    }

    pub fn extension(name: &str) -> TsExtension {
        TsExtension {
            reference: Self::reference(name),
        }
    }

    pub fn import(path: &str, reference: TsImportReference) -> TsImport {
        TsImport {
            dependency: TsDependencyIdent::Local(Self::path(path)),
            reference,
        }
    }

    pub fn import_default(path: &str, name: &str) -> TsImport {
        Self::import(path, TsImportReference::Default(name.into()))
    }

    pub fn import_glob(path: &str, name: &str) -> TsImport {
        Self::import(path, TsImportReference::Glob(name.into()))
    }

    pub fn import_named(path: &str, names: Vec<TsImportName>) -> TsImport {
        Self::import(path, TsImportReference::Named(names))
    }

    pub fn import_name(name: &str) -> TsImportName {
        TsImportName::Name(name.into())
    }

    pub fn import_alias(name: &str, alias: &str) -> TsImportName {
        TsImportName::Alias(name.into(), alias.into())
    }

    pub fn import_reference_default(name: &str) -> TsImportReference {
        TsImportReference::Default(name.into())
    }

    pub fn import_reference_glob(name: &str) -> TsImportReference {
        TsImportReference::Glob(name.into())
    }

    pub fn import_reference_named(names: Vec<TsImportName>) -> TsImportReference {
        TsImportReference::Named(names)
    }

    pub fn module(imports: Vec<TsImport>, definitions: Vec<TsDefinition>) -> TsModule {
        TsModule {
            doc: None,
            imports,
            definitions,
        }
    }

    pub fn alias<Type>(name: &str, descriptor: Type) -> TsAlias
    where
        Type: Into<TsDescriptor>,
    {
        TsAlias {
            doc: None,
            name: Self::identifier(name),
            descriptor: descriptor.into(),
        }
    }

    pub fn interface(name: &str, properties: Vec<TsProperty>) -> TsInterface {
        TsInterface {
            doc: None,
            name: Self::identifier(name),
            extensions: vec![],
            properties,
        }
    }

    pub fn branded(name: &str, primitive: TsPrimitive) -> TsBranded {
        TsBranded {
            doc: None,
            name: Self::identifier(name),
            primitive,
        }
    }

    pub fn embed_definition(name: &str, embed: &str) -> TsEmbedDefinition {
        TsEmbedDefinition {
            name: Self::identifier(name),
            embed: embed.into(),
        }
    }

    pub fn definition<Type>(descriptor: Type) -> TsDefinition
    where
        Type: Into<TsDefinition>,
    {
        descriptor.into()
    }

    pub fn property<Type>(name: &str, descriptor: Type) -> TsProperty
    where
        Type: Into<TsDescriptor>,
    {
        TsProperty {
            doc: None,
            name: Self::key(name),
            descriptor: descriptor.into(),
            required: true,
        }
    }

    pub fn property_optional<Type>(name: &str, descriptor: Type) -> TsProperty
    where
        Type: Into<TsDescriptor>,
    {
        TsProperty {
            required: false,
            ..Self::property(name, descriptor)
        }
    }

    pub fn object(properties: Vec<TsProperty>) -> TsObject {
        TsObject { properties }
    }

    pub fn array<Type>(descriptor: Type) -> TsArray
    where
        Type: Into<TsDescriptor>,
    {
        TsArray {
            descriptor: descriptor.into(),
        }
    }

    pub fn tuple(descriptors: Vec<TsDescriptor>) -> TsTuple {
        TsTuple { descriptors }
    }

    pub fn union(descriptors: Vec<TsDescriptor>) -> TsUnion {
        TsUnion { descriptors }
    }

    pub fn intersection(descriptors: Vec<TsDescriptor>) -> TsIntersection {
        TsIntersection { descriptors }
    }

    pub fn inline_import(path: &str, name: &str) -> TsInlineImport {
        TsInlineImport {
            path: Self::path(path),
            name: Self::identifier(name),
        }
    }

    pub fn record<Type>(key: TsRecordKey, descriptor: Type) -> TsRecord
    where
        Type: Into<TsDescriptor>,
    {
        TsRecord {
            key,
            descriptor: descriptor.into(),
        }
    }

    pub fn record_key_string() -> TsRecordKey {
        TsRecordKey::String
    }

    pub fn record_key_number() -> TsRecordKey {
        TsRecordKey::Number
    }

    pub fn record_key_boolean() -> TsRecordKey {
        TsRecordKey::Boolean
    }

    pub fn descriptor<Type>(descriptor: Type) -> TsDescriptor
    where
        Type: Into<TsDescriptor>,
    {
        descriptor.into()
    }

    pub fn any() -> TsAny {
        TsAny
    }
}
