use crate::prelude::internal::*;

impl TsConvert<TsRecordKey> for GtRecordKey {
    fn convert(&self, _context: &mut TsConvertContext) -> TsRecordKey {
        match self {
            GtRecordKey::String(_) => TsRecordKey::String,
            GtRecordKey::Number(_)
            | GtRecordKey::Int8(_)
            | GtRecordKey::Int16(_)
            | GtRecordKey::Int32(_)
            | GtRecordKey::IntU8(_)
            | GtRecordKey::IntU16(_)
            | GtRecordKey::IntU32(_)
            | GtRecordKey::Float32(_)
            | GtRecordKey::Float64(_) => TsRecordKey::Number,

            GtRecordKey::Int64(_)
            | GtRecordKey::Int128(_)
            | GtRecordKey::IntSize(_)
            | GtRecordKey::IntU64(_)
            | GtRecordKey::IntU128(_)
            | GtRecordKey::IntUSize(_) => TsRecordKey::String,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_node(GtRecordKey::String((0, 0).into())),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Number((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Int8((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Int16((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Int32((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntU8((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntU16((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntU32((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Float32((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Float64((0, 0).into())),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Int64((0, 0).into())),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::Int128((0, 0).into())),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntSize((0, 0).into())),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntU64((0, 0).into())),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntU128((0, 0).into())),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(GtRecordKey::IntUSize((0, 0).into())),
            @"String"
        );
    }
}
