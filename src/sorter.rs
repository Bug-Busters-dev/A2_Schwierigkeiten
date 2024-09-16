use std::collections::HashMap;

use crate::sorter_util;

pub fn sorter(file_number: u8) {
    let data: std::collections::HashMap<u8, String> =
        sorter_util::get_klausur_lines_data(file_number);

    let mut klausuren_vec_tmp: Vec<String> = Vec::new();
    for klausur in data {
        klausuren_vec_tmp.push(klausur.1);
    }

    let mut klausuren_vec: Vec<String> = Vec::new();
    for ctn in klausuren_vec_tmp {
        let ctn: String = ctn.replace(" < ", "");
        klausuren_vec.push(ctn);
    }

    println!("{:?}", klausuren_vec);

    let mut hash_vec: Vec<HashMap<char, u16>> = Vec::new();
    hash_vec.resize(klausuren_vec.len(), HashMap::new());

    for i in 0..klausuren_vec.len() {
        let mut char_count = 1;
        for char in klausuren_vec[i].chars() {
            hash_vec[i].entry(char).or_insert(char_count);
            char_count += 1;
        }
    }

    'hashmaps: for (i, hashmap) in hash_vec.clone().iter().enumerate() {
        println!("Hashmap for Klausur {}: {:?}", i, hashmap);

        // Check for duplicate keys across all hashmaps

        for j in 0..hash_vec.len() {
            for k in 0..hash_vec.len() {
                if j != k {
                    for (key1, val1) in hash_vec[j].clone().iter() {
                        'keys: for (key2, val2) in hash_vec[k].clone().iter() {
                            if key1 == key2 {
                                if val1 > val2 {
                                    let diff = val1 - val2;
                                    hash_vec[k].entry(*key2).or_insert(*val1);

                                    // adjjust other values
                                    'value_bigger: for (_, val) in hash_vec[k].iter_mut() {
                                        if *val > *val2 {
                                            *val += diff;
                                        } else {
                                            continue 'value_bigger;
                                        }
                                    }
                                } else if val2 > val1 {
                                    let diff = val2 - val1;
                                    hash_vec[k].entry(*key1).or_insert(*val2);

                                    // Adjust other values greater than the old smaller value
                                    'values_smaller: for (_, val) in hash_vec[j].iter_mut() {
                                        if *val == *val2 {
                                            continue 'values_smaller;
                                        }

                                        if *val > *val1 {
                                            *val += diff;
                                        } else {
                                            continue 'values_smaller;
                                        }
                                    }
                                }
                            } else {
                                continue 'keys;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", hash_vec)
}
