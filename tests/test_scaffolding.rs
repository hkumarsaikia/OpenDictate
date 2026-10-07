#[test]
fn test_rust_binary_compiles() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_opendictate_version() {
    assert_eq!(opendictate::VERSION, "2.0.0");
}
