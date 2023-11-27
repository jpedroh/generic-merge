use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn it_does_not_crash_and_produces_the_expected_output() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/smoke_java/base.java")
        .arg("--left-path=tests/samples/smoke_java/left.java")
        .arg("--right-path=tests/samples/smoke_java/right.java")
        .arg("--merge-path=tests/samples/smoke_java/merge.output.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/smoke_java/merge.expected.java").unwrap(),
        std::fs::read_to_string("tests/samples/smoke_java/merge.output.java").unwrap()
    )
}
