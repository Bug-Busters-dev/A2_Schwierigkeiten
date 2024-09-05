use std::{
    collections::HashMap,
    io::{self, Read},
};
pub mod reader;
pub mod sorter;
pub mod sorter_util;
const PATH: &str = "./data/schwierigkeiten0.txt";
fn main() {
    sorter::sorter(0);
    end();

    fn end() {
        println!("Press enter to exit");

        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).ok(); // ignore the result
    }
}
