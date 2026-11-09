impl Solution {
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut value_stack = Vec::new();
    for i in tokens{
        match i.as_str() {
            "+" => {
                let a = value_stack.pop().unwrap();
                let b = value_stack.pop().unwrap();
                value_stack.push(a+b);
            },
            "-" => {
                let b = value_stack.pop().unwrap();
                let a = value_stack.pop().unwrap();
                value_stack.push(a-b);
            },
            "*" => {
                let a = value_stack.pop().unwrap();
                let b = value_stack.pop().unwrap();
                value_stack.push(a*b);
            },
            "/" => {
                let b = value_stack.pop().unwrap();
                let a = value_stack.pop().unwrap();
                if a!= 0 && b!=0{

                value_stack.push(a/b)
                }else{
                    value_stack.push(0);
                }
            },
            _ =>{
                value_stack.push(i.parse::<i32>().unwrap());
            }
        }
    }
    value_stack.pop().unwrap()
}

}
