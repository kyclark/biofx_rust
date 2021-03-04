extern crate dna;
use std::process;

fn main() {
    let config = match dna::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = dna::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
