use colored::Colorize;
use itertools::Itertools;
use polars::prelude::*;
use std::env;
use std::vec;

fn get_conflicts(sorted_pairs: Vec<String>, char_pairs: Vec<Vec<String>>) {
    let mut is_conflict = false;
    // Für jedes `sorted_pairs` Element die `char_pairs` überprüfen
    for (i, sorted_pair) in sorted_pairs.iter().enumerate() {
        if i >= char_pairs.len() {
            println!("Fehler: Keine Daten in char_pairs für {}", sorted_pair);
            continue;
        }

        let current_pairs = &char_pairs[i];

        // Erstelle eine Häufigkeitskarte (frequency map) für die Elemente
        let mut haeufigkeit = std::collections::HashMap::new();

        for pair in current_pairs {
            let count = haeufigkeit.entry(pair).or_insert(0);
            *count += 1;
        }

        // Überprüfe, ob alle Paare gleich sind
        if haeufigkeit.len() == 1 {
            println!(
                "Alle Paare in Gruppe {} sind identisch: {:?}",
                sorted_pair, current_pairs
            );
        } else {
            // Bestimme die höchste Häufigkeit
            let max_count = haeufigkeit.values().cloned().max().unwrap_or(0);

            // Finde alle Einträge mit der maximalen Häufigkeit
            let most_common: Vec<_> = haeufigkeit
                .iter()
                .filter(|&(_, &count)| count == max_count)
                .map(|(pair, _)| (*pair).clone()) // Hier die doppelte Referenz auflösen
                .collect();

            let mut involved_chars: Vec<_> = haeufigkeit.keys().cloned().collect();
            involved_chars.sort();

            println!(
                "Fehler: Uneinheitliche Paare in Gruppe {}: Involvierte Buchstaben: {:?}, häufigste Elemente: {:?}",
                sorted_pair, involved_chars, most_common
            );
            is_conflict = true;
        }
    }
    if is_conflict {
        println!("{}", "Es gibt Konflikte in den Daten".red());
    } else {
        println!("Es gibt keine Konflikte in den Daten");
    }
}

pub fn make_df(klassenvec: Vec<String>) -> DataFrame {
    // make all char pairs for the first m letters into a series
    /* let m: u32 = sorter_util::get_n_m_k(&path_to_data, 2)
        .unwrap()
        .get_value()
        .unwrap();
    */
    env::set_var("POLARS_FMT_MAX_ROWS", "100");
    let mut pair_combinations: Vec<Vec<String>> = Vec::new();
    let mut klausur_numbers: Vec<u16> = Vec::new();
    // make a series for each line arbeit in klassenvec
    // file has m columns
    for (i, arbeit) in klassenvec.iter().enumerate() {
        let mut pairs: Vec<String> = Vec::new();
        // Erzeuge alle Kombinationen von zwei Buchstaben
        for (j, c1) in arbeit.chars().enumerate() {
            for c2 in arbeit.chars().skip(j + 1) {
                if c1 != c2 {
                    let pair = format!("{}{}", c1, c2);
                    pairs.push(pair);
                    klausur_numbers.push(i as u16 + 1 as u16);
                }
            }
        }

        let vec = Vec::from_iter(pairs);

        pair_combinations.push(vec.clone());
    }
    let pair_combinations = pair_combinations
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();

    // alphabeticly sorted pair combinations
    let alpha_sorted_pairs = pair_combinations
        .clone()
        .iter()
        .map(|x| x.chars().sorted().collect::<String>())
        .collect::<Vec<String>>();
    // let mut all_series: Vec<Series> = Vec::new();
    // make a series for each vec in vecvec
    //for (i, vec) in vecvec.iter().enumerate() {
    //    ljkjjet slice_vec: Vec<&str> = vec.iter().map(|x| x.as_str()).collect();
    //    let series = Series::new(format!("Arbeit_{}", i).into(), slice_vec);
    //    all_series.push(series);
    //}i
    //let max_len = all_series.iter().map(|s| s.len()).max().unwrap_or(0);
    //for series in all_series.iter_mut() {
    //    if series.len() < max_len {
    //        let pad_count = max_len - series.len();
    //        let padding: Vec<&str> = vec![""; pad_count];
    //        series.append(&Series::new("null".into(), padding)).unwrap();
    //    }
    //}
    // make a dataframe from all_series

    let char_series: Series = Series::new("CharPairs".into(), pair_combinations)
        .cast(&DataType::String)
        .unwrap();
    let sorted_series: Series = Series::new("SortedPairs".into(), alpha_sorted_pairs)
        .cast(&DataType::String)
        .unwrap();
    let klausur_series: Series = Series::new(
        "Klausur".into(),
        klausur_numbers
            .iter()
            .map(|&x| x as i32)
            .collect::<Vec<i32>>(),
    )
    .cast(&DataType::Int32)
    .unwrap();

    let df_pairs = DataFrame::new(vec![
        char_series.into_column(),
        sorted_series.into_column(),
        klausur_series.into_column(),
    ])
    .unwrap();
    println!("{:?}", df_pairs);
    df_pairs
}

pub fn locate_conflicts(_dataframe: DataFrame, klassenvec: Vec<String>) {
    let df = make_df(klassenvec);

    let dublicates = df
        .clone()
        .lazy()
        .group_by(["SortedPairs"])
        .agg(vec![
            col("SortedPairs").count().alias("count"),
            col("CharPairs"),
        ])
        .filter(col("count").gt(1))
        .collect()
        .unwrap();

    println!("{:?}", dublicates);
    // make dublicates to strings.
    /* let chars_pairs_series = dublicates
        .get_columns()
        .iter()
        .nth(0)
        .unwrap()
        .as_series()
        .unwrap()
        .to_owned();
    let strings: Vec<String> = chars_pairs_series
        .str()
        .unwrap()
        .into_iter()
        .filter_map(|opt_str| opt_str.map(|s| s.to_string()))
        .collect();
    */
    /*
        for every dublicate we wanna see if the char_series is in the same reinfolge wenn nt dann conflict und user warnen
    */

    // loop over SortedPairs column and look in CharPairs column in dubliactes
    let mut sorted_pairs: Vec<String> = Vec::new();
    let mut char_pairs: Vec<Vec<String>> = Vec::new();
    for sorted_pair in dublicates.column("SortedPairs").unwrap().str().unwrap() {
        let sorted_pair = sorted_pair.unwrap();
        sorted_pairs.push(sorted_pair.to_string());
    }
    println!("{:?}", char_pairs);
    for char_pair1 in dublicates.column("CharPairs").unwrap().list().unwrap() {
        let char_pair1 = char_pair1.unwrap();
        let char_pair1: Vec<String> = char_pair1
            .str()
            .unwrap()
            .into_iter()
            .map(|opt| opt.unwrap().to_string())
            .collect();
        char_pairs.push(char_pair1);
    }
    get_conflicts(sorted_pairs, char_pairs);
}

#[cfg(test)]
#[test]
fn main() {
    let klassenvec = vec![
        "FACG".to_string(),
        "BCA".to_string(),
        "EAB".to_string(),
        "BFA".to_string(),
    ];
    let df = make_df(klassenvec.clone());
    locate_conflicts(df, klassenvec);
}
#[test]
fn neu2() {
    let klassenvec = vec![
        "ABCDEJI".to_string(),
        "BCEDIHK".to_string(),
        "SGHIJ".to_string(),
        "GHSO".to_string(),
        "MNOK".to_string(),
        "KOM".to_string(),
        "PQRFMN".to_string(),
        "SFPNK".to_string(),
        "FTU".to_string(),
        "VWTZ".to_string(),
        "YXZT".to_string(),
        "ZWTVTU".to_string(),
        "KWZY".to_string(),
        "ABDEWZXYU".to_string(),
        "RQKL".to_string(),
        "PFKOXW".to_string(),
    ];
    let df = make_df(klassenvec.clone());
    locate_conflicts(df, klassenvec);
}
