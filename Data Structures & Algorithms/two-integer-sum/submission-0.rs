use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {            
            let diff = target - num;
            if let Some(&prev_value) = hash_table.get(&diff){
                return vec![prev_value as i32, i as i32];
            }
            hash_table.insert(num, i as i32);
        }
        vec![]
    }
}
