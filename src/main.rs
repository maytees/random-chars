extern crate rand;
use rand::Rng;

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide 1 argument - length of random string")
    }

    let len = match args.get(1) {
        Some(num) => num,
        None => panic!("No number!"),
    };

    for _iter in 0..len.clone().parse::<i64>().unwrap() {
        print!("{}", randomchar());
    }

    print!("\n\nCHAR COUNT - {}\n", len);

    io::stdout().flush().unwrap();
}

fn randomchar() -> char {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range(b'!'..b'~') as char;

    letter
}
