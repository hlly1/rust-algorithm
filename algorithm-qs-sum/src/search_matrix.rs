
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut x, mut y) = (0, matrix[0].len() - 1);
    while x < matrix.len(){
        if matrix[x][y] == target{
            return true;
        }else if matrix[x][y] < target{
            x+=1;
        }else if matrix[x][y] > target && y > 0{
            y-=1;
        }else{
            return false;
        }
    }
    return false;
}
