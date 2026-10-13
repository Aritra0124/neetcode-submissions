impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        let mut length = 1;
        let mut temp = 1;
        nums.sort_unstable();
        if nums.len() == 0{
            return 0
        }
        if nums.iter().all(|&c| c == 0){
            return 1
        }
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] ==  nums[i - 1] + 1 {
                temp += 1;
            }else{
            temp = 1;
        }
        length = temp.max(length);
        }
        length
    }
}
