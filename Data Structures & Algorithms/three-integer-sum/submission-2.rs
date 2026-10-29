impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let n = nums.len();
    if n < 3 {
        return result;
    }

    // 1. Sort the array to enable two-pointer traversal and easy duplicate skipping
    nums.sort();

    for i in 0..n - 2 {
        // Skip duplicate elements for the first number
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // Early exit optimizations
        if nums[i] + nums[i + 1] + nums[i + 2] > 0 {
            break; // Remaining numbers are too large
        }
        if nums[i] + nums[n - 2] + nums[n - 1] < 0 {
            continue; // This number is too small, try the next `i`
        }

        // 2. Two-pointer approach for the remaining subarray
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);

                // Skip duplicates for `left` and `right`
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1; // Sum is too small, move left pointer up
            } else {
                right -= 1; // Sum is too large, move right pointer down
            }
        }
    }

    result
}
}
