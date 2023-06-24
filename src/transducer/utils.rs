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

pub fn remove_from_or_delete(map: &mut HashMap<usize, HashMap<char, usize>>, q: &usize, ch: &char) {
    if let Some(trans) = map.get_mut(q) {
        if trans.get(ch).is_some() && trans.len() == 1 {
            map.remove(q);
        } else {
            trans.remove(ch);
        }
    }
}

pub fn vec_copy_take<T>(v: &Vec<T>, i: usize) -> Vec<T>
where
    T: Copy,
{
    let mut prefix = Vec::new();

    for j in 0..i {
        prefix.push(v[j]);
    }

    return prefix;
}

pub fn vec_copy_drop<T>(v: &Vec<T>, i: usize) -> Vec<T>
where
    T: Copy,
{
    let mut suffix = Vec::new();

    for j in i..v.len() {
        suffix.push(v[j]);
    }

    return suffix;
}
