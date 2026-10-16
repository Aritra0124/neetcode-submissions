impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    let mut new_string = s.to_lowercase().replace(" ", "").chars().filter(|c| !c.is_ascii_punctuation()).collect::<String>();
        let new_str = new_string.chars().rev().collect::<String>();
        println!("{:?} == {:?}", new_string, new_str);
        new_string == new_str
    }
}
