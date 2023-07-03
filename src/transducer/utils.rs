use std::collections::HashMap;

pub fn longest_common_prefix(w1: &Vec<char>, w2: &Vec<char>) -> Vec<char> {
    let mut lcp = Vec::new();

    for i in 0..w1.len() {
        if w1[i] == w2[i] {
            lcp.push(w1[i]);
        } else {
            break;
        }
    }

    return lcp;
}

pub fn add_to_or_insert(
    map1: &mut HashMap<usize, HashMap<char, usize>>,
    k1: usize,
    k2: char,
    v: usize,
) {
    match map1.get_mut(&k1) {
        Some(map2) => {
            map2.insert(k2, v);
        }
        None => {
            map1.insert(k1, HashMap::from([(k2, v)]));
        }
    }
}

pub fn remove_from_or_delete(
    map1: &mut HashMap<usize, HashMap<char, usize>>,
    k1: &usize,
    k2: &char,
) {
    if let Some(map2) = map1.get_mut(k1) {
        if map2.get(k2).is_some() && map2.len() == 1 {
            map1.remove(k1);
        } else {
            map2.remove(k2);
        }
    }
}
