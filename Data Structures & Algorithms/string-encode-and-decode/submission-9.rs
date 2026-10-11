impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
    let mut new_string = String::new();
    for i in strs {
        new_string.push_str(
            &format!("{}#{}", i.len(),i).to_string()
        );
    }
    new_string
}

pub fn decode(s: String) -> Vec<String> {
    let mut list_string: Vec<String> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i< s.len(){
        let temp: u8;
        if s.chars().nth(j).unwrap() != '#' {
            j +=1;
            continue;
        }else if s.chars().nth(j).unwrap() == '#'{
            temp = (&s[i..j]).parse::<u8>().unwrap();
            list_string.push(s[j+1..(j+1)+ temp as usize].to_string());
            i = j+ 1 + temp as usize;
            j = j + 1 + temp as usize;
        }
    }
    list_string

}
}
