use itertools::Itertools;
use polars::prelude::*;
use std::collections::HashSet;
use std::vec;

const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn find_conlficts(klassenvec: Vec<String>) {
    // make all char pairs for the first m letters into a series

    /* let m: u32 = sorter_util::get_n_m_k(&path_to_data, 2)
        .unwrap()
        .get_value()
        .unwrap();
    */
    let mut all_pairs: HashSet<String> = HashSet::new();
    let mut vecvec: Vec<Vec<String>> = Vec::new();
    // make a series for each line arbeit in klassenvec
    // file has m columns
    for arbeit in klassenvec {
        // Konvertiere den String in ein Set von einzigartigen Buchstaben
        let unique_chars: HashSet<char> = arbeit.chars().collect();
        let mut pairs: HashSet<String> = HashSet::new();
        // Erzeuge alle Kombinationen von zwei Buchstaben
        for (c1, c2) in unique_chars.iter().combinations(2).map(|v| (v[0], v[1])) {
            // Erstelle die Paare in alphabetischer Reihenfolge
            let pair = format!("{}{}", c1, c2);
            all_pairs.insert(pair.clone());
            // auch in umgekehrter Reihenfolge um AB und BA als gleich zu betrachten
            let reverse_pair = format!("{}{}", c2, c1);
            all_pairs.insert(reverse_pair);
            pairs.insert(pair);
        }
        let vec = Vec::from_iter(pairs.clone());
        vecvec.push(vec);
    }
    let mut all_series: Vec<Series> = Vec::new();
    // make a series for each vec in vecvec
    for (i, vec) in vecvec.iter().enumerate() {
        let slice_vec: Vec<&str> = vec.iter().map(|x| x.as_str()).collect();
        let series = Series::new(format!("Arbeit_{}", i).into(), slice_vec);
        all_series.push(series);
    }

    let max_len = all_series.iter().map(|s| s.len()).max().unwrap_or(0);

    for series in all_series.iter_mut() {
        if series.len() < max_len {
            let pad_count = max_len - series.len();
            let padding: Vec<&str> = vec![""; pad_count];
            series.append(&Series::new("null".into(), padding)).unwrap();
        }
    }

    // make a dataframe from all_series
    let df_pairs = DataFrame::new(
        all_series
            .iter()
            .map(|s| s.clone().into())
            .collect::<Vec<Column>>(),
    )
    .unwrap();

    println!("{:?}", df_pairs);
}

#[cfg(test)]
#[test]
fn main() {
    let klassenvec = vec![
        "FACG".to_string(),
        "BADC".to_string(),
        "EBA".to_string(),
        "BFA".to_string(),
    ];
    find_conlficts(klassenvec);
}
