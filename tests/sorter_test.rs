use schwierigkeiten::sorter::*;
use std::collections::HashMap;

#[test]
fn test_sortout_basic() {
    let map1 = HashMap::from([('A', 2), ('B', 2)]);
    let map2 = HashMap::from([('C', 5)]);
    let maps = vec![map1, map2];
    assert!(sortout(&maps) == "AB C" || sortout(&maps) == "BA C");
}

#[test]
fn test_sortout_with_multiple_maps() {
    let map1 = HashMap::from([('A', 2), ('B', 2), ('C', 5)]);
    let map2 = HashMap::from([('D', 1), ('E', 3)]);
    let maps = vec![map1, map2];
    assert!(sortout(&maps) == "D AB E C" || sortout(&maps) == "D BA E C" );
}

#[test]
fn test_sortout_with_unique_values() {
    let map1 = HashMap::from([('A', 3), ('B', 1)]);
    let map2 = HashMap::from([('C', 2)]);
    let maps = vec![map1, map2];
    assert_eq!(sortout(&maps), "B C A");
}

#[test]
fn test_sortout_empty_input() {
    let maps: Vec<HashMap<char, u16>> = vec![];
    assert_eq!(sortout(&maps), "");
}

#[test]
fn test_sortout_all_same_value() {
    let map1 = HashMap::from([('A', 2)]);
    let map2 = HashMap::from([('B', 2)]);
    let map3 = HashMap::from([('C', 2)]);
    let maps = vec![map1, map2, map3];
    assert_eq!(sortout(&maps), "ABC");
    assert!(
    sortout(&maps) == "ABC" || 
    sortout(&maps) == "ACB" ||
    sortout(&maps) == "BAC" ||
    sortout(&maps) == "BCA" ||
    sortout(&maps) == "CAB" ||
    sortout(&maps) == "CBA"
);
}

#[test]
fn test_sortout_single_map() {
    let map = HashMap::from([('A', 5), ('B', 2)]);
    let maps = vec![map];
    assert_eq!(sortout(&maps), "B A");
}
