use std::collections::{BTreeSet, HashMap};

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

pub fn remove_from_or_delete(map: &mut HashMap<usize, HashMap<char, usize>>, q: &usize, ch: &char) {
    if let Some(trans) = map.get_mut(q) {
        if trans.get(ch).is_some() && trans.len() == 1 {
            map.remove(q);
        } else {
            trans.remove(ch);
        }
    }
}

pub fn insert_or_push_in_partition(
    partitions: &mut Vec<BTreeSet<usize>>,
    value: usize,
    index: usize,
) {
    if index < partitions.len() {
        partitions[index].insert(value);
    } else {
        partitions.push(BTreeSet::from([value]));
    }
}
