impl Solution {
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
let mut stack = Vec::new();
    let mut days = vec![0; temperatures.len()];
    for (idx, &temp) in temperatures.iter().enumerate() {
        while let Some(&id) = stack.last(){
            if temp > temperatures[id]{
                days[id] = (idx - id) as i32;
                stack.pop();
            }else{
                break;
            }
        }
        stack.push(idx);
    }
    days
}
}
