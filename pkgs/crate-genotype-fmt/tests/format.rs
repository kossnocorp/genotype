use genotype_fmt::{GtfmtError, GtfmtFormatter};
use indoc::indoc;
use insta::assert_snapshot;

#[test]
fn module() {
    let result = GtfmtFormatter::format(indoc! {r#"
        //! Hello
        use ./foo/{A as B,C}
        X<T>:{a?:string|int,b:[T],...B}
        Y:{[string]:X<string>}
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        //! Hello

        use ./foo/{A as B, C}

        X<T>: { a?: string | int, b: [T], ...B, }

        Y: { [string]: X<string> }
    ");
}

#[test]
fn empty_module() {
    let result = GtfmtFormatter::format(indoc! {""}).unwrap();

    assert_snapshot!(result, @"");
}

#[test]
fn empty_collections() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Tuple: ( )
        Record: { [ ] : string }
        use ./module/{ }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Tuple: ()

        Record: { []: string }

        use ./module/{}
    ");
}

#[test]
fn short_union() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: string|int|null
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: string | int | null
    ");
}

#[test]
fn optional_union_pipes() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: | string | int |
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: string | int
    ");
}

#[test]
fn union_at_print_width() {
    // The formatted line is exactly 80 columns.
    let result = GtfmtFormatter::format(indoc! {r#"
        X: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA | Beta
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        X: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA | Beta
    ");
}

#[test]
fn union_above_print_width() {
    // One extra character makes the flat layout 81 columns.
    let result = GtfmtFormatter::format(indoc! {r#"
        X: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA | Beta
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        X:
          | AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
          | Beta
    ");
}

#[test]
fn wrapped_union() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Descriptor: DefinitionAlias | DefinitionInterface | DefinitionFunction | DefinitionVariable
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Descriptor:
          | DefinitionAlias
          | DefinitionInterface
          | DefinitionFunction
          | DefinitionVariable
    ");
}

#[test]
fn array_of_union_members() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Descriptors: [DefinitionAlias | DefinitionInterface | DefinitionFunction | DefinitionVariable]
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Descriptors: [
          | DefinitionAlias
          | DefinitionInterface
          | DefinitionFunction
          | DefinitionVariable
        ]
    ");
}

#[test]
fn record_of_union_members() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Definitions: { [string]: DefinitionAlias | DefinitionInterface | DefinitionFunction | DefinitionVariable }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Definitions: {
          [string]:
            | DefinitionAlias
            | DefinitionInterface
            | DefinitionFunction
            | DefinitionVariable
        }
    ");
}

#[test]
fn tuple() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Pair: (string , [int|null])
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Pair: (string, [int | null])
    ");
}

#[test]
fn wrapped_tuple() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Definitions: (DefinitionAlias, DefinitionInterface, DefinitionFunction, DefinitionVariable)
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Definitions: (
          DefinitionAlias,
          DefinitionInterface,
          DefinitionFunction,
          DefinitionVariable
        )
    ");
}

#[test]
fn generic_parameters_and_arguments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Pair<T , U>:{first:T,second:U}
        Example: Pair<string|int , [string]>
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Pair<T, U>: { first: T, second: U, }

        Example: Pair<string | int, [string]>
    ");
}

#[test]
fn wrapped_generic_arguments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Definitions: Collection<DefinitionAlias, DefinitionInterface, DefinitionFunction, DefinitionVariable>
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Definitions: Collection<
          DefinitionAlias,
          DefinitionInterface,
          DefinitionFunction,
          DefinitionVariable
        >
    ");
}

#[test]
fn imports_and_inline_imports() {
    let result = GtfmtFormatter::format(indoc! {r#"
        use ./definitions/{DefinitionAlias as Alias,DefinitionInterface,DefinitionFunction,DefinitionVariable}
        use ../shared/*
        Example: ./definitions/Pair<string,number>
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        use ./definitions/{
          DefinitionAlias as Alias,
          DefinitionInterface,
          DefinitionFunction,
          DefinitionVariable
        }
        use ../shared/*

        Example: ./definitions/Pair<string, number>
    ");
}

#[test]
fn annotations() {
    let result = GtfmtFormatter::format(indoc! {r#"
        #[marker]
        #[name="value"]
        #[args("value",1,true)]
        #[options(first=1,second="two")]
        Value: string
    "#})
    .unwrap();

    assert_snapshot!(result, @r#"
        #[marker]
        #[name = "value"]
        #[args("value", 1, true)]
        #[options(first = 1, second = "two")]
        Value: string
    "#);
}

#[test]
fn literal_spellings() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Text: "é🚀 // /* \" \u00e9"
        Number: +1_000 | -2.5e+3
        Flag: true | false
        Nothing: null
        Id: @string
    "#})
    .unwrap();

    assert_snapshot!(result, @r#"
        Text: "é🚀 // /* \" \u00e9"

        Number: +1_000 | -2.5e+3

        Flag: true | false

        Nothing: null

        Id: @string
    "#);
}

#[test]
fn rejects_missing_descriptor() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value:
    "#});

    assert!(matches!(result, Err(GtfmtError::Parse(_))));
}

#[test]
fn rejects_unclosed_object() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: {
    "#});

    assert!(matches!(result, Err(GtfmtError::Parse(_))));
}

#[test]
fn rejects_unclosed_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        /* unterminated
    "#});

    assert!(matches!(result, Err(GtfmtError::Parse(_))));
}

#[test]
fn rejects_invalid_escape() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: "bad\x"
    "#});

    assert!(matches!(result, Err(GtfmtError::Parse(_))));
}
