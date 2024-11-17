use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::{
    conflicts::{self},
    sorter_util::{self},
};

fn klausurenvec_bereinigen(mut klausuren: Vec<String>) -> Vec<String> {
    // Alle Paare sammeln, die in mindestens einer Klausur vorkommen
    let mut alle_paare = HashSet::new();
    for klausur in &klausuren {
        for pair in klausur.chars().collect::<Vec<_>>().windows(2) {
            let pair_str = pair.iter().collect::<String>();
            alle_paare.insert(pair_str);
        }
    }

    // Klausuren filtern: Behalte nur solche, die Paare mit mindestens einer anderen Klausur teilen
    klausuren.retain(|klausur| {
        let mut hat_geteilte_paare = false;

        // Überprüfe jedes Paar in der Klausur
        for pair in klausur.chars().collect::<Vec<_>>().windows(2) {
            let pair_str = pair.iter().collect::<String>();

            // Wenn das Paar mindestens ein weiteres Mal in `alle_paare` vorkommt, ist die Klausur gültig
            if alle_paare.contains(&pair_str) {
                hat_geteilte_paare = true;
                break;
            }
        }

        hat_geteilte_paare // Nur Klausuren mit geteilten Paaren bleiben erhalten
    });

    klausuren
}

pub fn update_hash_map(
    hash_map: &mut HashMap<char, u16>,
    kl_number: usize,
    kl_vec: &Vec<String>,
    cchar: char,
) -> () {
    let mut var_plus: u16 = 1;
    let mut chars_iter = kl_vec[kl_number].chars();
    let mut val: u16;

    'char: while let Some(c) = chars_iter.next() {
        if c == cchar {
            break 'char;
        }
    }

    for x in chars_iter {
        val = hash_map.get(&cchar).unwrap().clone();
        val += var_plus;
        var_plus += 1;
        hash_map.entry(x).and_modify(|v| *v = val);
    }
}

pub fn sorter(path: String) -> Vec<HashMap<char, u16>> {
    let mut klausuren_vec: Vec<String> = sorter_util::get_klausur_lines_data(&path);
    for klausur in klausuren_vec.iter_mut() {
        *klausur = klausur.replace(" < ", "");
    }

    let df = conflicts::make_df(klausuren_vec.clone());
    let mut klausuren_vec: Vec<String> =
        conflicts::locate_conflicts(df.clone(), klausuren_vec.clone());
    klausuren_vec.dedup();
    for klausur in klausuren_vec.iter_mut() {
        *klausur = klausur.chars().dedup().collect::<String>();
    }
    println!("{:?}", klausuren_vec);

    let klausuren_vec = klausurenvec_bereinigen(klausuren_vec);

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
                for (&cchar, &val_i) in hash_vec[i].clone().iter() {
                    if let Some(&val_j) = hash_vec[j].get(&cchar) {
                        if val_i != val_j {
                            let max_val: u16 = max(val_i, val_j);
                            hash_vec[i].insert(cchar, max_val);
                            hash_vec[j].insert(cchar, max_val);
                            changed = true;
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
