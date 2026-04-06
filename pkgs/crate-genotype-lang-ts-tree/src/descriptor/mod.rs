use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsDescriptor {
    Array(#[visit] Box<TsArray>),
    InlineImport(#[visit] TsInlineImport),
    Intersection(#[visit] TsIntersection),
    Literal(#[visit] TsLiteral),
    Object(#[visit] TsObject),
    Primitive(#[visit] TsPrimitive),
    Reference(#[visit] TsReference),
    Tuple(#[visit] TsTuple),
    Union(#[visit] TsUnion),
    Record(#[visit] Box<TsRecord>),
    Any(#[visit] TsAny),
}

impl TsDescriptor {
    pub fn scan_references(&self) -> TsDescriptorReferencesScanVisitor {
        let mut visitor = TsDescriptorReferencesScanVisitor::new();
        self.traverse(&mut visitor);
        visitor
    }
}

pub struct TsDescriptorReferencesScanVisitor {
    pub has_self_recursive: bool,
    pub has_forward: bool,
}

impl TsDescriptorReferencesScanVisitor {
    pub fn new() -> Self {
        TsDescriptorReferencesScanVisitor {
            has_self_recursive: false,
            has_forward: false,
        }
    }
}

impl TsVisitor for TsDescriptorReferencesScanVisitor {
    fn visit_reference(&mut self, node: &TsReference) {
        match node.rel {
            TsReferenceRel::SelfRecursive => self.has_self_recursive = true,
            TsReferenceRel::Forward => self.has_forward = true,
            TsReferenceRel::Regular => {}
        }
    }
}

impl From<&str> for TsDescriptor {
    fn from(str: &str) -> Self {
        TsDescriptor::Reference(str.into())
    }
}

impl From<TsAny> for TsDescriptor {
    fn from(any: TsAny) -> Self {
        TsDescriptor::Any(any)
    }
}

impl From<TsIntersection> for TsDescriptor {
    fn from(intersection: TsIntersection) -> Self {
        TsDescriptor::Intersection(intersection)
    }
}

impl From<TsObject> for TsDescriptor {
    fn from(object: TsObject) -> Self {
        TsDescriptor::Object(object)
    }
}

impl From<TsPrimitive> for TsDescriptor {
    fn from(primitive: TsPrimitive) -> Self {
        TsDescriptor::Primitive(primitive)
    }
}

impl From<TsReference> for TsDescriptor {
    fn from(reference: TsReference) -> Self {
        TsDescriptor::Reference(reference)
    }
}

impl From<TsUnion> for TsDescriptor {
    fn from(union: TsUnion) -> Self {
        TsDescriptor::Union(union)
    }
}

impl From<TsLiteral> for TsDescriptor {
    fn from(literal: TsLiteral) -> Self {
        TsDescriptor::Literal(literal)
    }
}

impl From<TsTuple> for TsDescriptor {
    fn from(tuple: TsTuple) -> Self {
        TsDescriptor::Tuple(tuple)
    }
}

impl From<TsRecord> for TsDescriptor {
    fn from(record: TsRecord) -> Self {
        TsDescriptor::Record(Box::new(record))
    }
}

impl From<TsArray> for TsDescriptor {
    fn from(array: TsArray) -> Self {
        TsDescriptor::Array(Box::new(array))
    }
}

impl From<TsInlineImport> for TsDescriptor {
    fn from(inline_import: TsInlineImport) -> Self {
        TsDescriptor::InlineImport(inline_import)
    }
}
