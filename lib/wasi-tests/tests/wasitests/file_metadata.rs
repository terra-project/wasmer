// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_file_metadata() {
    assert_wasi_output!(
        "../../wasitests/file_metadata.wasm",
        "file_metadata",
        vec![".".to_string(),],
        vec![],
        vec![],
        "../../wasitests/file_metadata.out"
    );
}
