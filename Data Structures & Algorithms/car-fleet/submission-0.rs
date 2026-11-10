impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    let mut stack: Vec<f64> = Vec::new();
    cars.sort_unstable_by(|a,b| b.0.cmp(&a.0));
    for (pos, spd) in cars{
        let time: f64 = ((target - pos) as f64) / spd as f64;
        if stack.is_empty() || Some(&time) > stack.last(){
            stack.push(time);
        }
    }
    stack.len() as i32
    }
}
