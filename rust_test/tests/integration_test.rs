//integration test
//to run the tests in specific file, `cargo test --test (file name)`
//to define helpers, recommend to use them with like `helpers/mod.rs` -- they're not printed in tests
extern crate rust_test;

#[test]
fn integrate_double() {
    assert_eq!(rust_test::double(5), 25);
}
