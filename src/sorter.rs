use std::collections::HashMap;

use crate::sorter_util;

#[allow(dead_code)]

fn resolve_conflicts(old_hash_vec: &Vec<HashMap<char, u16>>) -> &Vec<HashMap<char, u16>> {
    // conflicts sind wenn ein Value N einmal größer ist als ein Value M und einmal kleiner als ein Value M
    // User wird gefragt welcher größer sein soll
    // hash_vec wird entsprechend geupdated

    return old_hash_vec;
}

fn update_hash_map(hash_map: &mut HashMap<char, u16>, max_val: u16, old_val: u16) -> () {
    for (_, int) in hash_map.iter_mut() {
        if *int <= max_val {
            let diff = max_val - old_val;
            *int = *int + diff;
        }
    }
}
pub fn sorter(path: String) -> () {
    let data: HashMap<u8, String> = sorter_util::get_klausur_lines_data(&path);

    let mut klausuren_vec: Vec<String> = data.values().cloned().collect();
    for klausur in &mut klausuren_vec {
        *klausur = klausur.replace(" < ", "");
    }

    let mut hash_vec: Vec<HashMap<char, u16>> = vec![HashMap::new(); klausuren_vec.len()];

    for (i, klausur) in klausuren_vec.iter().enumerate() {
        for (j, char) in klausur.chars().enumerate() {
            hash_vec[i].insert(char, (j + 1) as u16);
        }
    }

    let mut changed: bool = true;
    while changed {
        changed = false;
        for i in 0..hash_vec.len() {
            for j in 0..hash_vec.len() {
                for (&char, &val_i) in hash_vec[i].clone().iter() {
                    if let Some(&val_j) = hash_vec[j].get(&char) {
                        println!("{:#?}", hash_vec);
                        if val_i != val_j {
                            let max_val: u16 = val_i.max(val_j);
                            hash_vec[i].insert(char, max_val);
                            hash_vec[j].insert(char, max_val);
                            changed = true;

                            if val_i == max_val {
                                update_hash_map(&mut hash_vec[j], max_val, val_j);
                            } else if val_j == max_val {
                                update_hash_map(&mut hash_vec[i], max_val, val_i);
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
