impl Solution {
pub fn is_valid(s: String) -> bool {
    if s.len() < 2{
        return false;
    }
    let mut stack = Vec::new();
    for i in s.chars(){
        if i == '(' ||  i == '{' || i == '[' {
            stack.push(i)
        }
        if i == ')' {
            if !stack.is_empty() &&stack.pop().unwrap() == '('{
                continue
            }else{
                return false;
            }
        }
        if i == '}' {
            if !stack.is_empty() &&stack.pop().unwrap() == '{'{
                continue
            }else{
                return false;
            }
        }
        if i == ']' {
            if !stack.is_empty() &&stack.pop().unwrap() == '['{
                continue
            }else{
                return false;
            }
        }
    }
    if stack.len() == 0{
        return true;
    }
    false
}
}
