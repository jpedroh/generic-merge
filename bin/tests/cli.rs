use std::process::Command;

use assert_cmd::prelude::*;

// #[test]
// fn if_there_is_a_conflict_it_returns_valid_exit_code() {
//     let mut cmd = Command::cargo_bin("generic-merge").unwrap();
//     cmd.arg("--base-path=tests/scenarios/smoke_java/base.java")
//         .arg("--left-path=tests/scenarios/smoke_java/left.java")
//         .arg("--right-path=tests/scenarios/smoke_java/right.java")
//         .arg("--merge-path=tests/scenarios/smoke_java/merge.output.java")
//         .arg("--language=java")
//         .assert()
//         .code(bin::SUCCESS_WITH_CONFLICTS);
// }
//
// #[test]
// fn if_there_is_no_conflict_it_returns_valid_exit_code() {
//     let mut cmd = Command::cargo_bin("generic-merge").unwrap();
//     cmd.arg("--base-path=tests/scenarios/no_conflicts/base.java")
//         .arg("--left-path=tests/scenarios/no_conflicts/left.java")
//         .arg("--right-path=tests/scenarios/no_conflicts/right.java")
//         .arg("--merge-path=tests/scenarios/no_conflicts/merge.output.java")
//         .arg("--language=java")
//         .assert()
//         .code(bin::SUCCESS_WITHOUT_CONFLICTS);
// }
