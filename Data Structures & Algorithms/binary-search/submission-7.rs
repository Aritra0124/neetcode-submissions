impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() as i32 - 1;
        while start <= end {
            let mid = start + (end - start) / 2;
            if target < nums[mid as usize]{
                end = mid -1;
            }else if target > nums[mid as usize]{
                start = mid +1 ;
            }else{
                return mid;
            }
        }
        -1
    }
}
