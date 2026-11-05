impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
      let mut left_pos: usize = 0;
    let mut right_pos = heights.len() - 1;
    let mut area = 0;
    for _ in 0..heights.len(){
        // println!("left pos: {}, right pos: {}, left_value: {}, right_value: {}, area: {}", left_pos, right_pos, heights[left_pos], heights[right_pos], area);
        if left_pos < right_pos {
            let temp_area = min(heights[left_pos], heights[right_pos]) * (right_pos - left_pos) as i32;

            if temp_area > area {
                area = temp_area;
            }
            if heights[left_pos] > heights[right_pos]{
                right_pos -= 1;
            }else{
                left_pos += 1;
            }
        }
    }
    area
}
}