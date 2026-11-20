use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut output = Vec::with_capacity(nums.len() - k + 1);
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(k);

        for i in 0..nums.len() {
            // 1. Remove indices that have fallen out of the left side of the window
            if let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                }
            }

            // 2. Remove smaller elements from the back (they can never be the max)
            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            // 3. Once the first full window of size `k` is formed, record the front max
            if i + 1 >= k {
                output.push(nums[*deque.front().unwrap()]);
            }
        }

        output
    }
}