extern crate clap;

use clap::{App, Arg};

#[derive(Debug, PartialEq)]
enum Base {
    A,
    C,
    T,
    G,
}

fn get_args() -> String {
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

    matches.value_of("dna").unwrap().to_string()
}

fn main() {
    let dna = get_args();
    let (count_a, count_c, count_g, count_t) = count(&dna);
    println!("{} {} {} {}", count_a, count_c, count_g, count_t);
}

/// Count the bases of DNA
fn count(dna: &String) -> (usize, usize, usize, usize) {
    let (mut count_a, mut count_c, mut count_g, mut count_t) = (0, 0, 0, 0);

    for base in dna.to_uppercase().chars().filter_map(|b| match_base(b)) {
        match base {
            Base::A => count_a += 1,
            Base::C => count_c += 1,
            Base::G => count_g += 1,
            Base::T => count_t += 1,
        }
    }

    (count_a, count_c, count_g, count_t)
}

/// Return Some(Base) or None
fn match_base(base: char) -> Option<Base> {
    if base == 'A' {
        Some(Base::A)
    } else if base == 'C' {
        Some(Base::C)
    } else if base == 'G' {
        Some(Base::G)
    } else if base == 'T' {
        Some(Base::T)
    } else {
        None
    }
}

#[test]
fn test_count() {
    assert_eq!(count(&"".to_string()), (0, 0, 0, 0));
    assert_eq!(count(&"A".to_string()), (1, 0, 0, 0));
    assert_eq!(count(&"C".to_string()), (0, 1, 0, 0));
    assert_eq!(count(&"G".to_string()), (0, 0, 1, 0));
    assert_eq!(count(&"T".to_string()), (0, 0, 0, 1));
    assert_eq!(count(&"accgggtttt".to_string()), (1, 2, 3, 4));
}

#[test]
fn test_match_base() {
    assert_eq!(match_base('A').unwrap(), Base::A);
    assert_eq!(match_base('C').unwrap(), Base::C);
    assert_eq!(match_base('G').unwrap(), Base::G);
    assert_eq!(match_base('T').unwrap(), Base::T);
    assert!(match_base('B').is_none());
}
