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
    let dna = args.dna;
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

    println!("{} {} {} {}", count_a, count_c, count_g, count_t);
}
