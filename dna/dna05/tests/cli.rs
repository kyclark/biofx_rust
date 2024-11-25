use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test1() -> Result<()> {
    let mut cmd = Command::cargo_bin("dna")?.arg("ACCGGGTTTT");
    cmd.unwrap()
        .assert()
        .stdout(predicate::eq(b"1 2 3 4\n" as &[u8]));

    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let mut cmd = Command::cargo_bin("dna")?
        .arg(dna)
        .unwrap()
        .assert()
        .stdout(predicate::eq(b"20 12 17 21\n" as &[u8]));

    Ok(())
}
