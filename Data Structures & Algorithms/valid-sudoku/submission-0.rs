use std::collections::HashMap;
impl Solution {
    pub fn vector_valid(list:Vec<char>)-> bool{
        println!("{:?}", list);
        let mut flag = true;
        let mut tracker = HashMap::new();
        list.iter().for_each(|&c|{
            if c!= '.'{
                if tracker.contains_key(&c) {
                    flag = false;
                }else{
                    tracker.entry(c).or_insert(1);
                }
            }
        });
        flag    
    }
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut flag = false;
        let column: Vec<Vec<char>> = (0..9).map(|j| (0..9).map(|i| board[i][j]).collect()).collect();
        let small_matrix: Vec<Vec<char>> = (0..9).map(|box_id| (0..9).map(|i|{
            let row_index = (box_id/3)*3 + i / 3;
            let col_index = (box_id%3)*3 + i % 3;
            board[row_index][col_index]
        }).collect()).collect();
        for i in 0..board.len(){
            // row wise validation && column wise validation && 3x3 matric validation
            if Self::vector_valid(board[i].clone()) && Self::vector_valid(column[i].clone()) && Self::vector_valid(small_matrix[i].clone()){
                flag = true;
            }else{
            flag = false;
            break;
            }
        }
        flag
    }
}
