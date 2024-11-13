use crate::reader::{self};
use crate::uniontype::UnionType;
use std::collections::HashMap;
use std::fmt::Error;
use std::string::String;

// const BASEPATH: &str = "./data/schwierigkeiten";

#[derive(Debug)]
pub enum SorterError {
    ReadError(std::io::Error),
    Other(String),
    InvalidReturnType,
}

impl From<std::io::Error> for SorterError {
    fn from(e: std::io::Error) -> Self {
        SorterError::ReadError(e)
    }
}

impl From<std::fmt::Error> for SorterError {
    fn from(e: std::fmt::Error) -> Self {
        SorterError::Other(format!("{}", e))
    }
}
impl From<String> for SorterError {
    fn from(s: String) -> Self {
        SorterError::Other(s)
    }
}
/// returntypes:
/// ```
///        1 => Ok(UnionType::Number(characters[0])),
///        2 => Ok(UnionType::Number(characters[1])),
///        3 => Ok(UnionType::Number(characters[2])),
///        4 => Ok(UnionType::Vec(characters)),
///        _ => Err(SorterError::InvalidReturnType),
/// ```
pub fn get_n_m_k(path: &String, returntype: u8) -> Result<UnionType, SorterError> {
    let characters: Result<Vec<char>, Error> = reader::read_carracters(&path, 1);

    let characters = match characters {
        Ok(chars) => chars,
        Err(e) => return Err(SorterError::from(e)),
    };

    let characters: Vec<u32> = characters.iter().filter_map(|&c| c.to_digit(10)).collect();

    match returntype {
        1 => Ok(UnionType::Number(characters[0])),
        2 => Ok(UnionType::Number(characters[1])),
        3 => Ok(UnionType::Number(characters[2])),
        4 => Ok(UnionType::Vec(characters)),
        _ => Err(SorterError::InvalidReturnType),
    }
}

pub fn get_klausur_lines_data(path: &String) -> Vec<String> {
    //sind von zeile 2 bis 1+n
    let n: UnionType = get_n_m_k(&path, 1).unwrap();
    let n = match n {
        UnionType::Number(n) => n,
        _ => todo!("this can happen, but you dont know how"),
    };

    let mut klausuren_data: Vec<String> = Vec::new();
    for x in 2..n + 2 {
        let mut output: String = String::new();
        reader::read_file_line(&mut output, &path, x.try_into().unwrap());
        klausuren_data.push(output);
    }
    return klausuren_data;
}
