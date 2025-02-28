/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

// 1 2 0
// 3 4 0
// 5 6 0
pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place
    // m * n -> n * m
    let mut row_begin = 0;
    let mut col_begin = 0;
    let m = matrix.len();
    let n = matrix[0].len();
    // expand
    let max = m.max(n);
    for _ in 0..max - m {
        matrix.push(vec![0; n]);
    }
    for i in 0..matrix.len() {
        for _ in 0..max - n {
            matrix[i].push(0);
        }
    }
    while row_begin < (max + 1) / 2 {
        while col_begin < (max + 1) / 2 {
            // row_begin = 0, col_begin = 1
            let tmp = matrix[row_begin][col_begin];
            // up
            matrix[row_begin][col_begin] = matrix[max - 1 - col_begin][row_begin];
            // left
            matrix[max - 1 - col_begin][row_begin] =
                matrix[max - 1 - row_begin][max - 1 - col_begin];
            // down
            matrix[max - 1 - row_begin][max - 1 - col_begin] =
                matrix[col_begin][max - 1 - row_begin];
            // right
            matrix[col_begin][max - 1 - row_begin] = tmp;
            col_begin += 1;
        }
        row_begin += 1;
    }
    // shrink
    let new_m = matrix.len();
    let new_n = matrix[0].len();
    for i in n..new_m {
        matrix.remove(i);
    }
    for i in 0..n {
        for j in m..new_n {
            matrix[i].remove(j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
    }
}
