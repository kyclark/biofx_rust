use anyhow::{anyhow, bail, Result};
use clap::Parser;
use rayon::prelude::*;
use std::{fs, ops::Range, path::Path};

#[derive(Debug, Parser)]
#[command(version, about)]
/// Count nucleotides
pub struct Args {
    /// DNA
    dna: String,
}

#[derive(Debug, PartialEq)]
struct BaseCount {
    a: usize,
    c: usize,
    t: usize,
    g: usize,
}

pub fn run(args: Args) -> Result<()> {
    let dna = read(&args.dna)?;
    let len = dna.len();
    if len == 0 {
        bail!("No input");
    }

    // Single-threaded
    //let ranges = vec![(0..len)];

    // Break into bits
    let mut breaks: Vec<_> = (0..len).step_by(10_000_000).collect();
    if breaks.last().map_or(false, |&last| last < len) {
        breaks.push(len);
    }
    let ranges: Vec<_> = breaks
        .windows(2)
        .flat_map(|v| {
            if let &[start, stop] = v {
                Some(start..stop)
            } else {
                None
            }
        })
        .collect();
    let counts: Vec<_> = ranges
        .into_par_iter()
        .map(|range| count(&dna, range))
        .collect();
    println!(
        "{} {} {} {}",
        counts.iter().map(|c| c.a).sum::<usize>(),
        counts.iter().map(|c| c.c).sum::<usize>(),
        counts.iter().map(|c| c.g).sum::<usize>(),
        counts.iter().map(|c| c.t).sum::<usize>(),
    );
    Ok(())
}

fn read(dna: &str) -> Result<Vec<u8>> {
    if Path::new(dna).is_file() {
        fs::read(dna).map_err(|e| anyhow!(e))
    } else {
        Ok(dna.as_bytes().to_vec())
    }
}

/// Count the bases of DNA
fn count(dna: &[u8], range: Range<usize>) -> BaseCount {
    let mut count = BaseCount {
        a: 0,
        c: 0,
        g: 0,
        t: 0,
    };

    for i in range {
        match dna[i] {
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
    let dna = b"ACCGGGTTTT".to_vec();
    assert_eq!(
        count(&dna, 0..3),
        BaseCount {
            a: 1,
            c: 2,
            g: 0,
            t: 0
        }
    );

    assert_eq!(
        count(&dna, 3..10),
        BaseCount {
            a: 0,
            c: 0,
            g: 3,
            t: 4
        }
    );

    assert_eq!(
        count(&dna, 0..10),
        BaseCount {
            a: 1,
            c: 2,
            g: 3,
            t: 4
        }
    );
}
