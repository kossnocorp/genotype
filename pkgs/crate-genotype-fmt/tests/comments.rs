use genotype_fmt::GtfmtFormatter;
use indoc::indoc;
use insta::assert_snapshot;

#[test]
fn documentation_annotations_and_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        //! module
        /// before
        #[tag = "a"]
        /// after
        X : { // opening
         a : string, /* middle */ b?: int // trailing
        , /// doc
         #[tag(foo=1,bar="x")] c: [string|int],
         /* dangling */ } // end
    "#})
    .unwrap();

    assert_snapshot!(result, @r#"
        //! module

        /// before
        #[tag = "a"]
        /// after
        X: {
          // opening
          a: string, /* middle */
          b?: int, // trailing
          /// doc
          #[tag(foo = 1, bar = "x")]
          c: [string | int],
          /* dangling */
        } // end
    "#);
}

#[test]
fn comments_only() {
    let result = GtfmtFormatter::format(indoc! {r#"
        // only
        /* block */
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        // only
        /* block */
    ");
}

#[test]
fn empty_tuple_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: (// empty
        )
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: (
          // empty
        )
    ");
}

#[test]
fn comments_around_colon() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value /* name */ : /* value */ string
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value /* name */ : /* value */ string
    ");
}

#[test]
fn block_comment_between_union_members() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: string /* first */ | int
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: string /* first */ | int
    ");
}

#[test]
fn line_comment_between_union_members() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: string | // next
        int
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value:
          | string // next
          | int
    ");
}

#[test]
fn leading_union_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: | /* leading */ string | int
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: | /* leading */ string | int
    ");
}

#[test]
fn leading_comment_on_single_union_member() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: | /* leading */ string
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: | /* leading */ string
    ");
}

#[test]
fn leading_union_comment_in_array() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: [| /* leading */ string]
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: [| /* leading */ string]
    ");
}

#[test]
fn array_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: [ // leading
        string // trailing
        ]
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: [
          // leading
          string // trailing
        ]
    ");
}

#[test]
fn generic_argument_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: Foo</* before */ string, int /* after */>
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: Foo</* before */ string, int /* after */>
    ");
}

#[test]
fn record_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: { [ /* key */ string /* key end */ ] /* colon */ : /* value */ string /* end */ }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: {
          [ /* key */ string /* key end */ ] /* colon */ : /* value */ string /* end */
        }
    ");
}

#[test]
fn import_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        use /* before */ ./module/{
         A /* before */ as /* after */ B, C // end
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        use /* before */ ./module/{
          A /* before */ /* after */ as B,
          C // end
        }
    ");
}

#[test]
fn union_member_annotations() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: /// doc
        #[tag] string | /// doc2
        int
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value:
          | /// doc
          #[tag]
          string
          | /// doc2
          int
    ");
}

#[test]
fn trailing_declaration_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        First: string // end
        Second: int
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        First: string // end

        Second: int
    ");
}

#[test]
fn nested_block_comment() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: string
        /* nested /* block */ comment */
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: string

        /* nested /* block */ comment */
    ");
}
