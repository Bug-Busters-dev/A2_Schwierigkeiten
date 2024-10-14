use std::io::{self, Read};
pub mod reader;
pub mod sorter;

pub mod sorter_util;
const PATH: &str = "./data/schwierigkeiten0.txt";
fn main() {
    sorter::sorter(0);
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
