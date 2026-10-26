use genotype_fmt::GtfmtFormatter;
use indoc::indoc;
use insta::assert_snapshot;

#[test]
fn module_documentation_before_annotated_alias() {
    let result = GtfmtFormatter::format(indoc! {r#"
        //! module
        //! more module documentation
        /// alias documentation
        #[tag]
        Value: string
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        //! module
        //! more module documentation

        /// alias documentation
        #[tag]
        Value: string
    ");
}

#[test]
fn module_documentation_before_imports() {
    let result = GtfmtFormatter::format(indoc! {r#"
        //! module
        use ./first/A
        use ./second/B
        Value: A | B
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        //! module

        use ./first/A
        use ./second/B

        Value: A | B
    ");
}

#[test]
fn aliases_and_attached_documentation() {
    let result = GtfmtFormatter::format(indoc! {r#"
        First: string
        /// second
        #[tag]
        Second: int
        Third: boolean
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        First: string

        /// second
        #[tag]
        Second: int

        Third: boolean
    ");
}

#[test]
fn aliases_with_trailing_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        First: string // first
        Second: int /* second */ // more
        /// third
        Third: boolean
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        First: string // first

        Second: int /* second */ // more

        /// third
        Third: boolean
    ");

    assert_eq!(GtfmtFormatter::format(&result).unwrap(), result);
}

#[test]
fn imports_before_documented_alias() {
    let result = GtfmtFormatter::format(indoc! {r#"
        use ./first/A
        use ./second/B
        /// combined
        #[tag]
        Value: A | B
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        use ./first/A
        use ./second/B

        /// combined
        #[tag]
        Value: A | B
    ");
}

#[test]
fn collapse_module_blank_lines() {
    let result = GtfmtFormatter::format(indoc! {r#"


        //! module



        First: string


        Second: int


    "#})
    .unwrap();

    assert_snapshot!(result, @"
        //! module

        First: string

        Second: int
    ");
}

#[test]
fn preserve_import_groups() {
    let result = GtfmtFormatter::format(indoc! {r#"
        use ./first/A
        use ./second/B


        use ./third/C
        Value: A | B | C
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        use ./first/A
        use ./second/B

        use ./third/C

        Value: A | B | C
    ");
}

#[test]
fn preserve_property_blank_line() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Asd: {
          qwe: 123,

          zxc: 456,
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Asd: {
          qwe: 123,

          zxc: 456,
        }
    ");

    assert_eq!(GtfmtFormatter::format(&result).unwrap(), result);
}

#[test]
fn collapse_property_blank_lines() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Asd: {
          qwe: 123,



          zxc: 456
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Asd: {
          qwe: 123,

          zxc: 456,
        }
    ");
}

#[test]
fn property_blank_lines_with_crlf() {
    let result = GtfmtFormatter::format(indoc! {
        "Asd: {\r\n  qwe: 123,\r\n \t\r\n\r\n  zxc: 456\r\n}\r\n"
    })
    .unwrap();

    assert_snapshot!(result, @"
        Asd: {
          qwe: 123,

          zxc: 456,
        }
    ");
}

#[test]
fn property_block_comment_groups() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: {
          first: string, /* trailing */


          /* second group */
          second: int,


          /* dangling */
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: {
          first: string, /* trailing */

          /* second group */
          second: int,

          /* dangling */
        }
    ");
}

#[test]
fn adjacent_properties_stay_together() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Asd: { qwe: 123, zxc: 456 }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Asd: { qwe: 123, zxc: 456, }
    ");
}

#[test]
fn property_documentation_and_trailing_comments() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: {
          first: string, // trailing


          /// second
          #[tag]
          second: int,

          // next group


          third: boolean
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: {
          first: string, // trailing

          /// second
          #[tag]
          second: int,

          // next group

          third: boolean,
        }
    ");
}

#[test]
fn nested_property_groups() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: {
          nested: { first: string,


            second: int },


          ...Base
        }
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: {
          nested: {
            first: string,

            second: int,
          },

          ...Base,
        }
    ");
}

#[test]
fn preserve_annotation_groups() {
    let result = GtfmtFormatter::format(indoc! {r#"
        /// documentation


        #[tag]

        Value: string
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        /// documentation

        #[tag]

        Value: string
    ");
}

#[test]
fn preserve_tuple_item_groups() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: (string,


        int)
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        Value: (
          string,

          int
        )
    ");
}

#[test]
fn preserve_comment_groups() {
    let result = GtfmtFormatter::format(indoc! {r#"
        // first


        /* second */


        // third
    "#})
    .unwrap();

    assert_snapshot!(result, @"
        // first

        /* second */

        // third
    ");
}

#[test]
fn literal_blank_lines_are_opaque() {
    let result = GtfmtFormatter::format(indoc! {r#"
        Value: "first


        second"
    "#})
    .unwrap();

    assert_snapshot!(result, @r#"
        Value: "first


        second"
    "#);
}
