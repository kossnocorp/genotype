#[test]
fn derive_visitor_explicit_fields() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/explicit_visit_pass.rs");
    t.compile_fail("tests/ui/explicit_visit_invalid_type.rs");
}
