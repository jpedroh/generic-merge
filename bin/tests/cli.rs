use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn it_does_not_crash_and_produces_the_expected_output() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    let predicate_fn = predicate::str::contains("public class Main  {  void delete  (  Pessoa pessoa ) ;  void create  (  Pessoa pessoa ) ;   public static void main  (   String  [ ] args )  {   int  x = 0 ;    System . out . println  ( <<<<<<<<< \"Hello, João!\" ========= \"Hello, Paulo!\" >>>>>>>>> ) ;   int  y = <<<<<<<<< 3 ========= 5 >>>>>>>>> ; }  void upsert  (  Pessoa pessoa ) ; }");
    cmd.assert().stdout(predicate_fn);
}
