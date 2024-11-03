use schwierigkeiten::sorter;
use std::env;
use std::io::{self, Write};
const DEFAULT_PATH: &str = "./data/schwierigkeiten0.txt";
const TEST_PATH: &str = "./data/test/test.txt";
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = args[1].parse().unwrap();
        sorter::sorter(path);
    } else {
        let path = TEST_PATH.parse().unwrap();
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    #[ignore]
    fn test_sorter() {
        sorter::sorter(DEFAULT_PATH.parse().unwrap());
    }
    #[test]
    fn test_end() {
        end();
    }
}
