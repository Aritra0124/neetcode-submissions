struct MinStack{
    value: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self{
            value: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.value.push(val);
    }

    pub fn pop(&mut self) {
        self.value.pop();
    }

    pub fn top(&self) -> i32 {
        self.value[self.value.len() - 1]
    }

    pub fn get_min(&self) -> i32 {
        *self.value.iter().min().unwrap()
    }
}
