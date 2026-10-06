impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            map.entry(chars).or_default().push(s);
        }
        map.into_values().collect()
    }
}