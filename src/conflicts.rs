use itertools::Itertools;
use polars::prelude::*;
use std::env;
use std::vec;

const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn make_df(klassenvec: Vec<String>) -> DataFrame {
    // make all char pairs for the first m letters into a series
    /* let m: u32 = sorter_util::get_n_m_k(&path_to_data, 2)
        .unwrap()
        .get_value()
        .unwrap();
    */
    env::set_var("POLARS_FMT_MAX_ROWS", "100");
    let all_pairs: Vec<String> = Vec::new();
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

fn _locate_conflicts(_dataframe: DataFrame, klassenvec: Vec<String>) {
    let df = make_df(klassenvec);
    let pairs = df.get_columns().iter().nth(1).unwrap();
    let pairs_series: Series = pairs.as_series().unwrap().to_owned();

    let dublicates = df
        .clone()
        .lazy()
        .group_by(["SortedPairs"])
        .agg(vec![col("SortedPairs").count().alias("count")])
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
    /*  for every dublicate we wanna see if the char_series is in the same reinfolge wenn nt dann conflict und user warnen
     */

    

}

#[cfg(test)]
#[test]
fn main() {
    let klassenvec = vec![
        "FACG".to_string(),
        "BCA".to_string(),
        "EBA".to_string(),
        "BFA".to_string(),
    ];
    let df = make_df(klassenvec.clone());
    _locate_conflicts(df, klassenvec);
}
