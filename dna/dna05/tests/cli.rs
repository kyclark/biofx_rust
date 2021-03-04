use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dna")?;

    cmd.arg("ACCGGGTTTT");
    cmd.unwrap()
        .assert()
        .stdout(predicate::eq(b"1 2 3 4\n" as &[u8]));

    Ok(())
}

#[test]
fn test2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dna")?;

    cmd.arg("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC")
        .unwrap()
        .assert()
        .stdout(predicate::eq(b"20 12 17 21\n" as &[u8]));

    Ok(())
}
