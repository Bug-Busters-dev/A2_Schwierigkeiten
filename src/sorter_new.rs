use std::{collections::HashMap, i16};

use crate::sorter_util;

pub fn sorter(file_number: u8) {
    let data: HashMap<u8, String> = sorter_util::get_klausur_lines_data(file_number);

    let mut klausuren_vec: Vec<String> = data.values().cloned().collect();
    for klausur in &mut klausuren_vec {
        *klausur = klausur.replace(" < ", "");
    }

    let mut hash_vec: Vec<HashMap<char, i16>> = vec![HashMap::new(); klausuren_vec.len()];

    for (i, klausur) in klausuren_vec.iter().enumerate() {
        for (j, char) in klausur.chars().enumerate() {
            hash_vec[i].insert(char, (j + 1) as i16);
        }
    }

    let mut changed: bool = true;
    while changed {
        changed = false;
        for i in 0..hash_vec.len() {
            for j in 0..hash_vec.len() {
                for (&key, &val_i) in hash_vec[i].clone().iter() {
                    if let Some(&val_j) = hash_vec[j].get(&key) {
                        if val_i != val_j {
                            let max_val: i16 = val_i.max(val_j);
                            hash_vec[i].insert(key, max_val);
                            hash_vec[j].insert(key, max_val);
                            changed = true;
                            if val_i == max_val {
                                for (_, int) in hash_vec[i].iter_mut() {
                                    let diff: i16 = *int - max_val;
                                    if !diff < 1 {
                                        *int = diff + 1;
                                    } else {
                                        continue;
                                    }
                                }
                            } else if val_j == max_val {
                                for (_, int) in hash_vec[j].iter_mut() {
                                    let diff = *int - max_val;
                                    if !diff < 1 {
                                        *int = diff + 1;
                                    } else {
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", hash_vec);
}
// tbs = ToBeSorted (genius)
pub fn process_sorted(_tbs: &mut Vec<HashMap<char, u16>>) {}
