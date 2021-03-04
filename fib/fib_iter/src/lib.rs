extern crate clap;

use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    generations: usize,
    litter: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fib")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Fibonacci sequence with litter")
        .arg(
            Arg::with_name("generations")
                .value_name("INT")
                .help("Number of generations")
                .short("n")
                .long("generations")
                .default_value("5"),
        )
        .arg(
            Arg::with_name("litter")
                .value_name("INT")
                .help("Size of litter")
                .short("k")
                .long("litter")
                .default_value("1"),
        )
        .get_matches();

    let n = matches
        .value_of("generations")
        .and_then(|n| n.trim().parse::<usize>().ok());

    let k = matches
        .value_of("litter")
        .and_then(|n| n.trim().parse::<usize>().ok());

    Ok(Config {
        generations: n.unwrap(),
        litter: k.unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let f = fibonacci(config.litter)
        .take(config.generations)
        .collect::<Vec<_>>();
    if let Some(v) = &f.last() {
        println!("{}", v);
    } else {
        println!("DOES NOT COMPUTE");
    }
    Ok(())
}

fn fibonacci(litter: usize) -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        let ret = state.1;
        state = (state.1 * litter, state.0 + state.1);
        Some(ret)
    })
}

#[test]
fn test_fibonacci() {
    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );
}
