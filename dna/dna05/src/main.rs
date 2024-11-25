use crate::{run, Args};
use clap::Parser;

fn main() {
    if let Err(e) = run(Args::parse()) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
