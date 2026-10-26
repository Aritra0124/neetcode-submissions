use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (pos, i) in numbers.iter().enumerate(){
        if map.get(&(target - i)).is_some() && pos > *map.get(&(target - i)).unwrap(){
            return vec![*map.get(&(target - i)).unwrap() as i32 + 1, pos as i32 + 1];
        }
        map.insert(i, pos);
    }
    vec![]
}
}
