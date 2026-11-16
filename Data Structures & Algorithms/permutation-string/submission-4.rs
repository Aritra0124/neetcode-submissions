    use std::collections::HashMap;

impl Solution {

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut tracker: HashMap<char, usize> = HashMap::new();
    for x in s1.chars() {
        *tracker.entry(x).or_insert(0) += 1;
    }

    let window = s1.len();

    for i in 0..=(s2.len() - window) {
        let slice = &s2[i..i + window];
        if tracker.iter().all(|(&c, &expected_count)| {
            slice.chars().filter(|&x| x == c).count() == expected_count
        }) {
            return true;
        }
    }

    false
}
}
