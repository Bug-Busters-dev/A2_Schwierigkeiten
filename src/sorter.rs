use crate::reader;
use crate::sorter_util;
pub fn sorter(file_number: u8) {
    sorter_util::get_klausur_lines_data(file_number);
}
