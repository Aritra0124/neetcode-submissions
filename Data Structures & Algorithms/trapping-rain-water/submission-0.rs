impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
let mut area = 0;
    let mut left = 0;
    let mut right = heights.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    while left < right{
        if heights[left] < heights[right] {
            if heights[left] >= left_max  {
                left_max = heights[left];
            } else {
                area += left_max - heights[left];
            }
            left += 1;
        }
        else{
            if heights[right] >= right_max{
                right_max = heights[right];
            }else{
                area += right_max - heights[right];
            }
            right -=1;
        }
        // print!("left: {left}, right: {right}, area :  {area}\n");
    }
    area
    }
}
 