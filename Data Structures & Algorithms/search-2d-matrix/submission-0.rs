impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.iter().any(|m| m.contains(&target)){
            return true;
        }
        return false
    }
}
