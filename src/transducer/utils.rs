pub struct Entry {
    pub word: Vec<char>,
    pub output: usize,
}

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
