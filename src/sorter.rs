use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use crate::sorter_util::{self};

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
fn resolve_conflicts(old_hash_vec: &Vec<HashMap<char, u16>>, path: &String) -> () {
    // conflicts sind wenn ein Value N einmal größer ist als ein Value M und einmal kleiner als ein Value M
    // User wird gefragt welcher größer sein soll
    // hash_vec wird entsprechend geupdated

    // finding conflicts
    {}
    // asking user

    {}
    println!("No conflicts found");
}

pub fn update_hash_map(
    hash_map: &mut HashMap<char, u16>,
    kl_number: usize,
    kl_vec: &Vec<String>,
    cchar: char,
) -> () {
    dbg!(&hash_map);
    println!("-----------------------------------------------");
    println!("found: {}", cchar);
    let mut var_plus: u16 = 1;
    let mut chars_iter = kl_vec[kl_number].chars();
    let mut val: u16;

    println!("chars iter before: {chars_iter:?}");
    'char: while let Some(c) = chars_iter.next() {
        if c == cchar {
            break 'char;
        }
    }
    println!("chars iter  after: {chars_iter:?}");

    for x in chars_iter {
        val = dbg!(hash_map.get(&cchar).unwrap().clone());
        val += dbg!(var_plus);
        var_plus += 1;
        hash_map.entry(x).and_modify(|v| *v = val);
        println!("modified: {}, to be {}", x, val);
    }
    println!("-----------------------------------------------");
}

pub fn sorter(path: String) -> Vec<HashMap<char, u16>> {
    let mut klausuren_vec: Vec<String> = sorter_util::get_klausur_lines_data(&path);
    dbg!(&klausuren_vec);
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
                                println!("vali was maxval");
                            } else if val_j == max_val {
                                update_hash_map(&mut hash_vec[i], i, &klausuren_vec, cchar);
                                println!("valj was maxval");
                            }
                        }
                    }
                }
            }
        }
    }
    return hash_vec;
}

// tbs = ToBeSorted (genius)
// sorts the hashmaps for the value of the keys
// from lowest to highest
// only keys
// output exmaple:
// "B A C D"
// if B: 1, A: 2, C: 3, D: 4
pub fn sortout(tbs: &Vec<HashMap<char, u16>>) -> String {
    // Create a new HashMap to hold the grouped keys by values
    let mut value_to_keys: HashMap<u16, Vec<String>> = HashMap::new();

    // Populate the value_to_keys HashMap
    for map in tbs {
        for (key, value) in map {
            value_to_keys
                .entry(*value)
                .or_insert_with(Vec::new)
                .push(key.to_string());
        }
    }

    // Create a sorted vector from the keys and sort it by value
    let mut sorted_keys: Vec<(u16, String)> = value_to_keys
        .iter()
        .map(|(&value, keys)| {
            let grouped_keys = keys.join(""); // Join keys with the same value
            (value, grouped_keys)
        })
        .collect();

    // Sort the vector by value
    sorted_keys.sort_by_key(|(val, _)| *val);

    // Create the final output string
    let output: String = sorted_keys
        .into_iter()
        .map(|(_, keys)| keys)
        .collect::<Vec<_>>()
        .join(" ");
    let output = dedup_str(&output);
    // Print the output
    output
}
fn dedup_str(input: &str) -> String {
    let mut seen = HashSet::new();
    let mut result = String::new();

    for ch in input.chars() {
        if seen.insert(ch) || ch == ' ' {
            // insert returns false if the character was already present
            result.push(ch);
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sortout() {
        let mut tbs: Vec<HashMap<char, u16>> = vec![
            vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)]
                .into_iter()
                .collect(),
            vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)]
                .into_iter()
                .collect(),
            vec![('A', 1), ('B', 2), ('C', 3), ('D', 4)]
                .into_iter()
                .collect(),
        ];
        let output = &sortout(&mut tbs);
        assert_eq!(output, "A B C D");
    }
}
