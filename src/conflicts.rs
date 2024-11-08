use itertools::Itertools;
// use polars::prelude::*;
use std::collections::{HashMap, HashSet};

const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct ValuePair {
    value1: (char, i16),
    value2: (char, i16),
}
fn sort_alphapeticaly(charpair: &str) -> String {
    let mut chars: Vec<char> = charpair.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    chars.into_iter().collect<String>()
}

fn to_struct(charpair: &str, hashmaps: Vec<HashMap<char, u16>>, index: usize) -> ValuePair{
    
    for char in charpair.chars() {
        let value1 = hashmaps[index].get(&char).unwrap();
        let value2 = hashmaps[index].get(&char).unwrap();
    }



}

fn find_conlficts(klausurenvec: Vec<String>, hash_vec: Vec<HashMap<char, u16>>, ) {

    for (i, klausur) in klausurenvec.iter().enumerate() {


        for (j, chars) in klausur.chars().enumerate() {
            hash_vec[i]
        }
    }






    let mut all_pairs: HashSet<String> = HashSet::new();
    let mut allklaururpairs: Vec<Vec<String>> = Vec::new();
    // make a series for each line arbeit in klassenvec
    // file has m columns
    for arbeit in klausurenvec {
        // Konvertiere den String in ein Set von einzigartigen Buchstaben
        let unique_chars: HashSet<char> = arbeit.chars().collect();
        let mut pairs: HashSet<String> = HashSet::new();
        // Erzeuge alle Kombinationen von zwei Buchstaben
        for (i, (c1, c2)) in unique_chars
            .iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .enumerate()
        {
            // Erstelle die Paare in alphabetischer Reihenfolge
            let pair = format!("{}{}", c1, c2);
            all_pairs.insert(pair.clone());
            // auch in umgekehrter Reihenfolge um AB und BA als gleich zu betrachten
            let reverse_pair = format!("{}{}", c2, c1);
            all_pairs.insert(reverse_pair);
            pairs.insert(pair);
        }
        let klausurpairs = Vec::from_iter(pairs.clone());
        allklaururpairs.push(klausurpairs);
    }

    // make a series for each vec in vecvec
    for (i, vec) in allklaururpairs.iter().enumerate() {
        let slice_vec: Vec<&str> = vec.iter().map(|x| x.as_str()).collect();
        let series = Series::new(format!("Arbeit_{}", i).into(), slice_vec);
        // create dataframe from series and value series
        unimplemented!();
    }

    // let max_len = all_series.iter().map(|s| s.len()).max().unwrap_or(0);

    // for series in all_series.iter_mut() {
  /*   if series.len() < max_len {
        let pad_count = max_len - series.len();
        let padding: Vec<&str> = vec![""; pad_count];
        series.append(&Series::new("null".into(), padding)).unwrap();
    }
}
*/
// make a dataframe from all_series
/*  let df_pairs = DataFrame::new(
    all_series
        .iter()
        .map(|s| s.clone().into())
        .collect::<Vec<Column>>(),
)
.unwrap();
*/
// println!("{:?}", df_pairs);
 }


#[cfg(test)]
mod tests {
#[test]
#[ignore]
fn main() {
    let klassenvec = vec![
        "FACG".to_string(),
        "BADC".to_string(),
        "EBA".to_string(),
        "BFA".to_string(),
    ];
    find_conlficts(klassenvec);
}
#[test]
fn test_sort_alphapeticaly() {
    assert_eq!(sort_alphapeticaly("BA"), "AB");
    assert_eq!(sort_alphapeticaly("AB"), "AB"); 
    }
}