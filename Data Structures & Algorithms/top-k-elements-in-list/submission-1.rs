use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut list_counter: HashMap<&_, i32> = HashMap::new();
    for i in nums.iter() {
        *list_counter.entry(i).or_default() += 1;
    }
    let mut counter = Vec::new();
    for i in 0..k{
        let temp = **list_counter.iter().max_by_key(|&(_, v)| v).unwrap().0;
        counter.push(temp);
        list_counter.remove(&temp);
    }
   counter
    }
}
