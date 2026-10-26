use genotype_fmt::GtfmtFormatter;
use indoc::indoc;
use insta::assert_snapshot;

#[test]
fn empty_object() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {}
    ");
}

#[test]
fn final_property_comma() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { value: string }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { value: string, }
    ");
}

#[test]
fn existing_final_property_comma() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { value: string, }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { value: string, }
    ");
}

#[test]
fn single_line_multiple_properties() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { first: string, second: int, third: boolean }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { first: string, second: int, third: boolean, }
    ");
}

#[test]
fn missing_spaces() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object:{first:string,second:int,third?:boolean}
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { first: string, second: int, third?: boolean, }
    ");
}

#[test]
fn inconsistent_spaces() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object : {   first :string ,second:   int,   third?:boolean,   }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { first: string, second: int, third?: boolean, }
    ");

    assert_eq!(GtfmtFormatter::format(&result).unwrap(), result);
}

#[test]
fn single_property_at_print_width() {
    // The formatted line, including the trailing comma and brace, is 80 columns.
    let result = GtfmtFormatter::format(indoc! {r#"
        Object:{value:PropertyTypeWithALongNameThatFitsExactlyWithinLineWidthLimit}
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { value: PropertyTypeWithALongNameThatFitsExactlyWithinLineWidthLimit, }
    ");
}

#[test]
fn single_property_above_print_width_after_spacing() {
    // The input fits, but spacing and the trailing comma make it 81 columns.
    let result = GtfmtFormatter::format(indoc! {r#"
        Object:{value:PropertyTypeWithALongNameThatFitsExactlyWithinLineWidthLimits}
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {
          value: PropertyTypeWithALongNameThatFitsExactlyWithinLineWidthLimits,
        }
    ");

    assert_eq!(GtfmtFormatter::format(&result).unwrap(), result);
}

#[test]
fn single_long_property() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { value: VeryLongPropertyTypeNameThatStillFitsOnItsOwnButExceedsTheObjectLine }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {
          value: VeryLongPropertyTypeNameThatStillFitsOnItsOwnButExceedsTheObjectLine,
        }
    ");
}

#[test]
fn multiple_properties_above_print_width() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { first: FirstPropertyWithALongTypeName, second: SecondPropertyWithALongTypeName }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {
          first: FirstPropertyWithALongTypeName,
          second: SecondPropertyWithALongTypeName,
        }
    ");
}

#[test]
fn multiline_object_stays_expanded() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: {
          first:string, second:int
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {
          first: string,
          second: int,
        }
    ");
}

#[test]
fn closing_brace_on_separate_line_stays_expanded() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { value:string
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {
          value: string,
        }
    ");
}

#[test]
fn optional_final_property_comma() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { first: string, last?: int }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { first: string, last?: int, }
    ");
}

#[test]
fn final_spread_comma() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { value: string, ...Base }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: { value: string, ...Base, }
    ");
}

#[test]
fn final_property_comma_before_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Object: { value: string // trailing
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Object: {
          value: string, // trailing
        }
    ");
}

#[test]
fn wrapped_property_union() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Definition: {
          descriptor: DefinitionAlias | DefinitionInterface | DefinitionFunction | DefinitionVariable
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Definition: {
          descriptor:
            | DefinitionAlias
            | DefinitionInterface
            | DefinitionFunction
            | DefinitionVariable,
        }
    ");
}

#[test]
fn nested_objects() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Definition:{...Base,source:{file:string,line?:u32},exports:[{name:string,type?:DefinitionId}]}
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Definition: {
          ...Base,
          source: { file: string, line?: u32, },
          exports: [{ name: string, type?: DefinitionId, }],
        }
    ");
}

#[test]
fn union_of_objects() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Result: { value: string } | { error: string }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Result: { value: string, } | { error: string, }
    ");
}

#[test]
fn multiline_string() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: { text: "first
        second", kind: string }
    "#})
    .unwrap();

    assert_snapshot!(result, @r#"
        Value: {
          text: "first
        second",
          kind: string,
        }
    "#);
}

#[test]
fn empty_object_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: { /* empty */ }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: { /* empty */ }
    ");
}

#[test]
fn trailing_property_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: { x: string // trailing
        , y: int }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: {
          x: string, // trailing
          y: int,
        }
    ");
}

#[test]
fn comment_before_nested_object() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: { x: /* comment */ { y: string } }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: { x: /* comment */ { y: string, }, }
    ");
}
