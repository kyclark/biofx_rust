extern crate clap;

use clap::{App, Arg};

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

    for base in dna.to_uppercase().chars() {
        if base == 'A' {
            count_a += 1;
        } else if base == 'C' {
            count_c += 1;
        } else if base == 'G' {
            count_g += 1;
        } else if base == 'T' {
            count_t += 1;
        }
    }

    (count_a, count_c, count_g, count_t)
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
