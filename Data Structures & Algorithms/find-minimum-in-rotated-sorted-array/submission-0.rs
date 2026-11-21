impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let k = nums.iter().min().unwrap();
        *k
    }
}
