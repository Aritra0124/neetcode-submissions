use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() == t.len(){
            let mut table = HashMap::new();
            let mut t_table = HashMap::new();
            for i in s.chars() {
                if table.get(&i).is_none() {
                    table.insert(i, 1);
                }else{
                    let counter = table.get(&i).unwrap() + 1;
                    table.insert(i, counter);
                }
            }
            for i in t.chars() {
                println!("{:?}", i);
                if t_table.get(&i).is_none() {
                    t_table.insert(i, 1);
                }else{
                    let counter = t_table.get(&i).unwrap() + 1;
                    t_table.insert(i, counter);
                }
            }
            if table == t_table{
                return true
            }
        }
        return false
    }
}
