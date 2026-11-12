use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut tracker = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;
        for (right, c) in s.chars().enumerate(){
            if let Some(&prev_id) = tracker.get(&c){
                if prev_id >= left{
                left = prev_id + 1;
            }
            }
            tracker.insert(c, right);
            max_len = max(max_len, (right - left +1) as i32);
        }
        max_len
    }
}
