
use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;


type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args()-> TestResult{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}


fn runs(args: &[&str], outfile: &str)-> TestResult{
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1()-> TestResult {
    runs(&["Hello there"],"tests/expected/hello1.txt")
}

#[test]
fn hello2()-> TestResult {
    runs(&["Hello", "there"],"tests/expected/hello2.txt")
}

#[test]
fn hello3()-> TestResult {
    runs(&["Hello  there", "-n"],"tests/expected/hello1.n.txt")
}

#[test]
fn hello4()-> TestResult {
    runs(&["-n", "Hello", "there"],"tests/expected/hello2.n.txt")
}




