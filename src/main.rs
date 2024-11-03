use std::env;
use std::io::{self, Write};
use schwierigkeiten::sorter;

const DEFAULT_PATH: &str = "./data/schwierigkeiten1.txt";
const TEST_PATH: &str = "./data/test/test.txt";
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = args[1].parse().unwrap();
        sorter::sorter(path);
    } else {
        println!("No path provided, using default path: \rdata/schwierigkeiten0.txt");
        press_enter(); 

        let path = DEFAULT_PATH.parse().unwrap();
        sorter::sorter(path);
    }
    end();
}
fn press_enter() {
    println!("Press enter to continue");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    if buffer == "\n" {
        return;
    }
}
fn end() {
    println!("Ending program");
    press_enter();
    std::process::exit(0); 
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
