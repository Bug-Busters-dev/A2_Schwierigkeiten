use std::env;
use std::io::{self, Read};

pub mod reader;
pub mod sorter;

pub mod sorter_util;
const DEFAULT_PATH: &str = "./data/schwierigkeiten0.txt";
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = args[1].parse().unwrap();
        sorter::sorter(path);
    } else {
        let path = DEFAULT_PATH.parse().unwrap();
        sorter::sorter(path);
    }
    end();
}

fn end() {
    println!("Press enter to exit");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    if buffer == "\n" {
        std::process::exit(0);
    }
}
