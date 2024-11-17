use colored::Colorize;
use itertools::Itertools;
use polars::prelude::*;
use std::env;
use std::io;
use std::vec;

fn replace_pairs(klausur: &str, pair1: &str, pair2: &str) -> String {
    let mut result = klausur.to_string();
    let first_char_p2 = pair2.chars().nth(0).unwrap();
    let second_char_p2 = pair2.chars().nth(1).unwrap();

    let first_char_p1 = pair1.chars().nth(0).unwrap();
    let second_char_p1 = pair1.chars().nth(1).unwrap();

    // wenn first_char_p1 und second_char_p1 in klausur sind (in richtiger Reihenfolge)
    // nutze iter um zu überprüfen ob die chars in der richtigen Reihenfolge sind
    if klausur.chars().position(|c| c == first_char_p2)
        > klausur.chars().position(|c| c == second_char_p2)
    {
        // wenn pair1 in klausur ist
        // only replace first occurence
        result = result.replacen(first_char_p2, &second_char_p2.to_string(), 1);
        // only replace second occurence of the char (nn would replace 2. n)
        result = result.replacen(first_char_p1, &second_char_p1.to_string(), 1);

        // Check if first_char_p1 and second_char_p1 are in `klausur` in the correct order
    }
    result

    // pair 2 ist das erwünschte pair
}

fn get_conflicts(
    sorted_pairs: Vec<String>,
    char_pairs: Vec<Vec<String>>,
    klassenvec: &mut Vec<String>,
) -> Vec<String> {
    // Überprüfe jede Gruppe in `sorted_pairs` mit den entsprechenden `char_pairs`
    for (i, sorted_pair) in sorted_pairs.iter().enumerate() {
        if i >= char_pairs.len() {
            println!("Fehler: Keine Daten in `char_pairs` für {}", sorted_pair);
            continue;
        }

        let current_pairs = &char_pairs[i];

        // Sammle alle involvierten Paare ohne Frequenz
        let mut involved_chars: Vec<_> = current_pairs.iter().cloned().collect();
        involved_chars.sort();
        involved_chars.dedup(); // Entfernt Duplikate, sodass jedes involvierte Paar nur einmal vorkommt

        // Überprüfe, ob alle Paare gleich sind
        if involved_chars.len() == 1 {
            println!(
                "Alle Paare in Gruppe {} sind identisch: {:?}",
                sorted_pair, current_pairs
            );
        } else {
            println!(
                "Fehler: Uneinheitliche Paare in Gruppe {}: Involvierte Paare: {:?}",
                sorted_pair, involved_chars
            );
            // print the current data with < between chars
            println!("Jetztige daten:");
            println!("-----------------");
            for line in klassenvec.iter() {
                // highlight the conflicting pairs
                for c in line.chars() {
                    if involved_chars[1].contains(&c.to_string())
                        || involved_chars[0].contains(&c.to_string())
                    {
                        print!(" {} ", c.to_string().red());
                    } else {
                        print!(" {} ", c);
                    }
                }
                println!();
            }
            println!("-----------------");
            loop {
                let change_type: u8;
                loop {
                    println!("Du kannst ein buchstabe löschen (1 eintippen), oder die anordnung der buchstaben ändern (2 eintippen)");
                    let mut val = String::new();
                    io::stdin()
                        .read_line(&mut val)
                        .expect("Fehler beim lesen der Eingabe");
                    match val.trim().parse::<u8>() {
                        Ok(val) => {
                            change_type = val;
                            break;
                        }
                        Err(_) => {
                            println!("Bitte geben Sie eine gültige Zahl ein.");
                        }
                    }
                }

                // Benutzer nach dem bevorzugten Paar fragen
                if change_type == 2 {
                    let mut preferred_pair = String::new();
                    println!("Bitte gebe deine Gewünschte reihenfolge ein");
                    io::stdin()
                        .read_line(&mut preferred_pair)
                        .expect("Fehler beim Lesen der Eingabe");

                    let mut preferred_pair = preferred_pair.trim().to_string();

                    // Sicherstellen, dass die Eingabe gültig ist
                    if !involved_chars.contains(&&preferred_pair) {
                        println!("Ungültige Eingabe. Bitte versuchen Sie es erneut.");

                        // für das gleiche Paar erneut fragen
                        loop {
                            preferred_pair.clear();
                            io::stdin()
                                .read_line(&mut preferred_pair)
                                .expect("Fehler beim Lesen der Eingabe");
                            let preferred_pair = preferred_pair.trim().to_string();
                            dbg!(&preferred_pair);
                            if involved_chars.contains(&preferred_pair) {
                                break;
                            } else {
                                println!("Ungültige Eingabe. Bitte versuchen Sie es erneut.");
                            }
                        }
                    }

                    // `klausuren`-Vektor aktualisieren, indem alle involvierten Paare auf das bevorzugte Paar gesetzt werden
                    for entry in klassenvec.iter_mut() {
                        println!("Vorherige Klausur: {}", entry);
                        for pair in &involved_chars {
                            *entry =
                                replace_pairs(entry, pair.as_str(), dbg!(&preferred_pair.trim()));
                        }
                        println!("Aktualisierte Klausur: {}", entry);
                        println!("----------------------------")
                    }

                    println!("Der aktualisierte klausuren-Vektor ist: {:?}", klassenvec);
                    break;
                } else if change_type == 1 {
                    println!("Welcher buchstabe und in welcher klausur (Zähle ab 1) zB. a1 wäre klausur 1 der buchstabe a");
                    let mut output = String::new();
                    io::stdin()
                        .read_line(&mut output)
                        .expect("Fehler beim Lesen der Eingabe");
                    if let Some((char_to_remove, index_str)) = output.split_once(' ') {
                        if let Ok(index) = index_str.parse::<usize>() {
                            if index > 0 && index <= klassenvec.len() {
                                let char_to_remove = char_to_remove.trim();
                                let klausur = &mut klassenvec[index - 1];
                                *klausur = klausur.replace(char_to_remove, "");
                                println!(
                                    "Der Buchstabe {} wurde aus der Klausur {} entfernt.",
                                    char_to_remove, index
                                );
                            }
                        }
                        break;
                    }
                } else {
                    println!("Please write 1 or 2");
                    continue;
                }
            }
        }
    }
    return klassenvec.to_owned();
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

    df_pairs
}

pub fn locate_conflicts(_dataframe: DataFrame, klassenvec: Vec<String>) -> Vec<String> {
    let df = make_df(klassenvec.clone());

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

    // loop over SortedPairs column and look in CharPairs column in dubliactes
    let mut sorted_pairs: Vec<String> = Vec::new();
    let mut char_pairs: Vec<Vec<String>> = Vec::new();
    for sorted_pair in dublicates.column("SortedPairs").unwrap().str().unwrap() {
        let sorted_pair = sorted_pair.unwrap();
        sorted_pairs.push(sorted_pair.to_string());
    }
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
    return get_conflicts(sorted_pairs, char_pairs, &mut klassenvec.clone());
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
#[test]
fn neu3() {
    let klassenvec = vec![
        "HSCGA".to_string(),
        "SOJLF".to_string(),
        "MODXU".to_string(),
        "SCENM".to_string(),
        "EMFXB".to_string(),
        "DGPXA".to_string(),
        "RLXUT".to_string(),
        "MOFVD".to_string(),
        "SOUTP".to_string(),
        "ZQKXI".to_string(),
        "BWIY".to_string(),
    ];
    let df = make_df(klassenvec.clone());
    locate_conflicts(df, klassenvec);
}
