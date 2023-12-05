use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn if_there_is_a_conflict_it_returns_valid_exit_code() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/smoke_java/base.java")
        .arg("--left-path=tests/samples/smoke_java/left.java")
        .arg("--right-path=tests/samples/smoke_java/right.java")
        .arg("--merge-path=tests/samples/smoke_java/merge.java")
        .assert()
        .code(bin::SUCCESS_WITH_CONFLICTS);
}

#[test]
fn if_there_is_no_conflict_it_returns_valid_exit_code() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/no_conflicts/base.java")
        .arg("--left-path=tests/samples/no_conflicts/left.java")
        .arg("--right-path=tests/samples/no_conflicts/right.java")
        .arg("--merge-path=tests/samples/no_conflicts/merge.java")
        .assert()
        .code(bin::SUCCESS_WITHOUT_CONFLICTS);
}
