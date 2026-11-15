use std::collections::HashMap;
use std::cmp::max;

impl Solution {
pub fn character_replacement(s: String, k: i32) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut count = [0; 26];
    let mut max_freq = 0;
    let mut start = 0;
    let mut max_len = 0;

    for end in 0..chars.len() {
        let idx = (chars[end] as u8 - b'A') as usize;
        count[idx] += 1;
        
        max_freq = std::cmp::max(max_freq, count[idx]);

        let window_len = (end - start + 1) as i32;
        if window_len - max_freq > k {
            let left_idx = (chars[start] as u8 - b'A') as usize;
            count[left_idx] -= 1;
            start += 1;
        }

        max_len = std::cmp::max(max_len, (end - start + 1) as i32);
    }

    max_len
}
}
