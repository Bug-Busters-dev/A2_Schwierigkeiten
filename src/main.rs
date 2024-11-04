use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use schwierigkeiten::sorter;

#[allow(unused)]
const DEFAULT_PATH: &str = "./data/schwierigkeiten1.txt";
const TEST_PATH: &str = "./data/test/test.txt";

#[allow(unused_assignments)]
fn main() {
    let mut hashvec: Vec<HashMap<char, u16>> = Vec::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = args[1].parse().unwrap();
        hashvec = sorter::sorter(path);
    } else {
        println!("No path provided, using default path: \ndata/schwierigkeiten0.txt");
        press_enter(); 

        let default_path = TEST_PATH.parse().unwrap();
        hashvec = sorter::sorter(default_path);
    }
    //let output = sorter::sortout(&mut hashvec);
    let output = hashvec;
    println!("----------------------------------");
    println!("{:?}", output);
    println!("----------------------------------");
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
