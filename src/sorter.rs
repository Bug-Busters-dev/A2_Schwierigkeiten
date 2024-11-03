use std::{cmp::max, collections::HashMap};

use crate::sorter_util::{self, UnionType};

const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
fn resolve_conflicts(old_hash_vec: &Vec<HashMap<char, u16>>, path: &String) -> () {
    // conflicts sind wenn ein Value N einmal größer ist als ein Value M und einmal kleiner als ein Value M
    // User wird gefragt welcher größer sein soll
    // hash_vec wird entsprechend geupdated

    // finding conflicts
    for (i, char) in ABC.chars().enumerate() {
        let m = sorter_util::get_n_m_k(&path, 1).unwrap();
        let m = match m {
            UnionType::Number(n) => n,
            _ => todo!("this can happen, but you dont know how"),
        };
        if i > m.try_into().unwrap() {
            break;
        }

        for char1 in ABC.chars() {
            for j in 0..old_hash_vec.len() {
                for k in 0..old_hash_vec.len() {
                    for l in 0..old_hash_vec.len() {
                        if let Some(&val) = old_hash_vec[j].get(&char) {
                            if let Some(&val1) = old_hash_vec[k].get(&char1) {
                                if let Some(&val2) = old_hash_vec[l].get(&char) {
                                    println!("testing: {} {} {}", char, char1, char);
                                    println!("Values: {} {} {}", val, val1, val2);
                                    println!("---------------------------------");
                                    if val > val1 && val < val2 && val1 == val2 {
                                        println!("Conflict found: {}", char);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // asking user
        }
    }
    println!("No conflicts found");
}

pub fn update_hash_map(
    hash_map: &mut HashMap<char, u16>,
    kl_number: usize,
    kl_vec: &Vec<String>,
    cchar: char,
) -> () {
    let mut var_plus: u16 = 1;
    let mut chars_iter = kl_vec[kl_number].chars();
    let mut val: u16 = 0;

    /* while let Some(c) = chars_iter.next() {
         if c == cchar {
            index = kl_vec[kl_number]
                .chars()
                .position(|x| x == c)
                .unwrap()
                .try_into()
                .unwrap();
        }


    }  */
    println!("chars iter before: {chars_iter:?}");
    'char: while let Some(c) = chars_iter.next() {
        if c == cchar {
            chars_iter.next();
            break 'char;
        }
    }
    println!("chars iter  after: {chars_iter:?}");
    /* let _ = chars_iter.map(|x| {
        val = hash_map.get(&x).unwrap().clone();
        val += var_plus;
        var_plus += 1;
        hash_map.entry(x).and_modify(|v| *v = val);
        val = 0;
    });*/
    for x in chars_iter {
       val += hash_map.get(&x).unwrap().clone();
        val += var_plus;
        var_plus += 1;
        hash_map.entry(x).and_modify(|v| *v = val);
        val = 0;
    }
}

pub fn sorter(path: String) -> () {
    let data: HashMap<u8, String> = sorter_util::get_klausur_lines_data(&path);
    println!("{:?}", data);

    let mut klausuren_vec: Vec<String> = data.values().cloned().collect();
    for klausur in klausuren_vec.iter_mut() {
        *klausur = klausur.replace(" < ", "");
    }
    println!("klausurenvec :  {:?}", klausuren_vec);
    let mut hash_vec: Vec<HashMap<char, u16>> = vec![HashMap::new(); klausuren_vec.len()];
    for (i, klausur) in klausuren_vec.iter().enumerate() {
        for (j, char) in klausur.chars().enumerate() {
            hash_vec[i].insert(char, (j + 1) as u16);
        }
    }
    println!("before: {:?}", hash_vec);

    let mut changed: bool = true;
    while changed {
        changed = false;
        for i in 0..hash_vec.len() {
            for j in 0..hash_vec.len() {
                for (&cchar, &val_i) in hash_vec[i].clone().iter() {
                    if let Some(&val_j) = hash_vec[j].get(&cchar) {
                        // println!("{:#?}", hash_vec);

                        if val_i != val_j {
                            let max_val: u16 = max(val_i, val_j);
                            hash_vec[i].insert(cchar, max_val);
                            hash_vec[j].insert(cchar, max_val);
                            changed = true;
                            // println!("Hasvec: {:#?}", hash_vec);

                            if val_i == max_val {
                                update_hash_map(&mut hash_vec[j], j, &klausuren_vec, cchar);
                            } else if val_j == max_val {
                                update_hash_map(&mut hash_vec[i], i, &klausuren_vec, cchar);
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
