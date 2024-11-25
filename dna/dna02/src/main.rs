use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
/// Count nucleotides
struct Args {
    /// DNA
    dna: String,
}

fn main() {
    let args = Args::parse();
    let (count_a, count_c, count_g, count_t) = count(&args.dna);
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
