use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn it_does_not_crash_and_produces_the_expected_output() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    let predicate_fn = predicate::str::contains("public interface Repository  {  void create  (  Pessoa pessoa ) ;  void delete  (  Pessoa pessoa ) ;  void remove  (  Pessoa pessoa ) ;  void insert  (  Pessoa pessoa ) ; }");
    cmd.assert().stdout(predicate_fn);
}
