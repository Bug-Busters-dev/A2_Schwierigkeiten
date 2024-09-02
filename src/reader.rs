use core::fmt;
use std::{fmt::Error, fs::File, io::Read};
pub fn read_file_line(output: &mut String, path: &String, line: usize) {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    for (i, line_ctn) in contents.lines().enumerate() {
        if i == line - 1 {
            output.clear();
            output.push_str(line_ctn);
            break;
        } else {
            continue;
        }
    }
}

pub fn read_carracters(path: &str, line: usize) -> Result<Vec<char>, fmt::Error> {
    let mut file = File::open(path).expect("couldnt read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to open file");

    if contents.is_empty() {
        return Err(Error);
    }

    let mut result = Vec::new();

    for (i, line_ctn) in contents.lines().enumerate() {
        if i == line - 1 {
            result.extend(line_ctn.chars());
        }
    }

    Ok(result)
}
