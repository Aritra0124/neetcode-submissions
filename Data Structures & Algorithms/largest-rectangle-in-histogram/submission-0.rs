impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let mut heights = heights;
    
    // Append a 0 at the end to flush out any remaining elements in the stack
    heights.push(0);

    for (i, &h) in heights.iter().enumerate() {
        while let Some(&top_idx) = stack.last() {
            if h < heights[top_idx] {
                stack.pop();
                let height = heights[top_idx];
                
                // Calculate width: if stack is empty, it extends all the way to the left (0)
                // Otherwise, it spans from the new stack top + 1 to i - 1
                let width = match stack.last() {
                    Some(&left_idx) => (i - left_idx - 1) as i32,
                    None => i as i32,
                };

                let area = height * width;
                if area > max_area {
                    max_area = area;
                }
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
    }
}
