/*
   問題: M*Nの行列について、要素が0であれば、その行と列の全てを0にするようなアルゴリズム。
*/

use std::collections::HashSet;

fn set_zero_when_the_line_has_zero(mut matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    if matrix.len() < 1 {
        return matrix
    }
    // boolの配列でもよかった。
    // 1行目と1列目を使うことで空間計算量の削減が可能。
    // HashSet: p.374
    let mut zero_row = HashSet::new();
    let mut zero_col = HashSet::new();
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 0 {
                zero_row.insert(row);
                zero_col.insert(col);
            }
        }
    }
    for row in zero_row {
        for i in 0..matrix[0].len() {
            matrix[row][i] = 0;
        }
    }
    for col in zero_col {
        for i in 0..matrix.len() {
            matrix[i][col] = 0;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_zero_when_the_line_has_zero() {
        let matrix = vec![
            vec![1,2,3,0,5],
            vec![6,7,0,9,10],
            vec![11,12,13,14,15]
        ];
        let matrix_set = set_zero_when_the_line_has_zero(matrix);
        let expect_matrix = vec![
            vec![0,0,0,0,0],
            vec![0,0,0,0,0],
            vec![11,12,0,0,15]
        ];
        assert_eq!(matrix_set, expect_matrix);

        let empty_vec = vec![vec![]];
        let expected_empty_vec = empty_vec.clone();
        assert_eq!(set_zero_when_the_line_has_zero(empty_vec), expected_empty_vec);
    }
}

fn main() {
    let matrix = vec![
        vec![1,2,3,0,5],
        vec![6,7,0,9,10]
    ];
    set_zero_when_the_line_has_zero(matrix);
}
