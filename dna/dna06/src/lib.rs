use anyhow::{anyhow, Result};
use clap::Parser;
use std::{fs::read_to_string, path::Path};

#[derive(Debug, Parser)]
#[command(version, about)]
/// Count nucleotides
pub struct Args {
    /// DNA
    dna: String,
}

#[derive(Debug, PartialEq)]
struct BaseCount {
    a: u32,
    c: u32,
    t: u32,
    g: u32,
}

pub fn run(args: Args) -> Result<()> {
    let counts = count(read(&args.dna)?);
    println!("{} {} {} {}", counts.a, counts.c, counts.g, counts.t);
    Ok(())
}

fn read(dna: &str) -> Result<String> {
    if Path::new(dna).is_file() {
        read_to_string(dna).map_err(|e| anyhow!(e))
    } else {
        Ok(dna.to_string())
    }
}

/// Count the bases of DNA
fn count(dna: String) -> BaseCount {
    let mut count = BaseCount {
        a: 0,
        c: 0,
        g: 0,
        t: 0,
    };

    for base in dna.bytes() {
        match base {
            b'a' | b'A' => count.a += 1,
            b'c' | b'C' => count.c += 1,
            b'g' | b'G' => count.g += 1,
            b't' | b'T' => count.t += 1,
            _ => (),
        }
    }

    count
}

#[test]
fn test_count() {
    assert_eq!(
        count(""),
        BaseCount {
            a: 0,
            c: 0,
            g: 0,
            t: 0
        }
    );

    assert_eq!(
        count("ACCGGGTTTT"),
        BaseCount {
            a: 1,
            c: 2,
            g: 3,
            t: 4
        }
    );
}
