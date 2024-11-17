use schwierigkeiten::sorter::{self, sortout};
use schwierigkeiten::{reader, sorter_util};
use std::collections::HashMap;
use std::io::{self, Write};
use std::{env, path};

#[allow(unused)]
const DEFAULT_PATH: &str = "./data/schwierigkeiten0.txt";
#[allow(unused)]
const TEST_PATH: &str = "./data/test/test.txt";

#[allow(unused_assignments)]
fn main() {
    let mut hashvec: Vec<HashMap<char, u16>> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let mut pub_path = String::new();
    if args.len() > 1 {
        let path: String = args[1].parse().unwrap();
        println!("Path provided: \n{}", path);
        pub_path = path.clone();
        hashvec = sorter::sorter(path);
    } else {
        println!("No path provided, using default path: \n{}", DEFAULT_PATH);
        press_enter();

        let default_path: String = DEFAULT_PATH.parse().unwrap();
        pub_path = default_path.clone();
        hashvec = sorter::sorter(default_path);
    }
    //let output = sorter::sortout(&mut hashvec);
    let output = hashvec;

    let output = sortout(&output);
    let m = sorter_util::get_n_m_k(&DEFAULT_PATH.parse().unwrap(), 1).unwrap();
    m.get_value::<u32>();

    let mut chars_to_return = String::new();
    let anzahl_klausuren = sorter_util::get_n_m_k(&pub_path, 1).unwrap();
    let anzahl_klausuren = anzahl_klausuren.get_value::<u32>().unwrap();
    reader::read_file_line(
        &mut chars_to_return,
        &pub_path,
        (anzahl_klausuren + 2).try_into().unwrap(),
    );
    let output: String = output
        .chars()
        .filter(|c| chars_to_return.contains(*c))
        .collect();

    println!("{:?}", output);
    if "./output/output.txt".parse::<path::PathBuf>().is_ok() {
        std::fs::remove_file("./output/output.txt").expect("Unable to remove file");
    }
    if !"./output".parse::<path::PathBuf>().is_ok() {
        std::fs::create_dir_all("./output").expect("Unable to create directory");
    }
    std::fs::write("./output/output.txt", output).expect("Unable to write file");
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    #[ignore]
    fn test_sorter() {
        sorter::sorter(DEFAULT_PATH.parse().unwrap());
    }
}
