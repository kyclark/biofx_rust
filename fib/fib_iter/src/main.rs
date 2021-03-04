extern crate fib;
use std::process;

fn main() {
    let config = match fib::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fib::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
