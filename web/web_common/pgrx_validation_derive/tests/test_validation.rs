//! Test submodule to check that the derive macro attribute `validation` causes
//! compilation errors upon invalid usage.

#[test]
/// Tests whether the `validation` attribute raises the appropriate compilation errors.
fn test_illegal_validation() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/test_private_validation.rs");
    t.compile_fail("tests/ui/test_generics_validation.rs");
	t.compile_fail("tests/ui/test_empty_return_type_validation.rs");
    t.compile_fail("tests/ui/test_no_args_validation.rs");
    t.compile_fail("tests/ui/test_return_bool_validation.rs");
    t.compile_fail("tests/ui/test_invalid_error_validation.rs");
    t.compile_fail("tests/ui/test_almost_correct_validation.rs");
    t.compile_fail("tests/ui/test_not_must_validation.rs");
    t.compile_fail("tests/ui/test_camelcased_validation.rs");
    t.compile_fail("tests/ui/test_async_validation.rs");
    t.compile_fail("tests/ui/test_no_doc_validation.rs");
    t.pass("tests/ui/test_correct_validation.rs");
}