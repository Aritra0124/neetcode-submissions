impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    let new_string = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
        let new_str = new_string.chars().rev().collect::<String>();
        println!("{:?} == {:?}", new_string, new_str);
        new_string == new_str
    }
}
