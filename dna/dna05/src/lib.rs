extern crate clap;

use clap::{App, Arg};
use std::collections::HashMap;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    dna: String,
}

#[derive(Debug, PartialEq)]
struct BaseCount {
    a: u32,
    c: u32,
    t: u32,
    g: u32,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("dna")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Tetranucleotide frequency")
        .arg(
            Arg::with_name("dna")
                .value_name("DNA")
                .help("Input DNA")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    Ok(Config {
        dna: matches.value_of("dna").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let counts = count(&config.dna);
    println!("{} {} {} {}", counts.a, counts.c, counts.g, counts.t);
    Ok(())
}

/// Count the bases of DNA
fn count(dna: &String) -> BaseCount {
    let mut count: HashMap<char, u32> = HashMap::new();
    for base in dna.to_uppercase().chars() {
        let num = count.entry(base).or_insert(0);
        *num += 1;
    }

    BaseCount {
        a: *count.get(&'A').unwrap_or(&0),
        c: *count.get(&'C').unwrap_or(&0),
        g: *count.get(&'G').unwrap_or(&0),
        t: *count.get(&'T').unwrap_or(&0),
    }
}

#[test]
fn test_count() {
    assert_eq!(
        count(&"".to_string()),
        BaseCount {
            a: 0,
            c: 0,
            g: 0,
            t: 0
        }
    );

    assert_eq!(
        count(&"ACCGGGTTTT".to_string()),
        BaseCount {
            a: 1,
            c: 2,
            g: 3,
            t: 4
        }
    );
}
