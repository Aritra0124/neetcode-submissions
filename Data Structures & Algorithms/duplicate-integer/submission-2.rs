use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut tracker = HashMap::new();
        for i in nums.iter(){
            if tracker.get(i).is_none(){
                tracker.insert(*i, 1);
            }else{
                return true;
            }
        }
        false
    }
}
