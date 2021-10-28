/*
   問題: N*Nの行列に描かれた、1つのピクセルが4バイト四方の画像がある。その画像を90度回転させるメソッドを書く。追加領域なしでできるか？
*/
// !!このfnは解答を見た上で作成
// 4つの組ごとに回転を繰り返す
fn matrix_rotation(mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // チェック
    if matrix.len() == 0 || matrix.len() != matrix[0].len(){
        return matrix
    }
    let n = matrix.len();
    let mut top;
    for layer in 0..(n / 2) {
        // 1辺の長さ分さらにループする
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            top = matrix[first][i];

            matrix[first][i]           = matrix[last-offset][first];
            matrix[last-offset][first] = matrix[last][last-offset];
            matrix[last][last-offset]  = matrix[i][last];
            matrix[i][last]            = top;
        }
    }
    matrix
}

// 4byte = 32bit
// vec, slice : p.56
fn matrix_rotation_with_flatten_vec(vec_to_rotate: Vec<u32>) -> Vec<u32> {
    let vec_len = vec_to_rotate.len();
    // https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
    let n = (vec_len as f64).sqrt() as usize;
    let mut row = 0;
    let mut col = 0;
    let mut new_row = 0;
    let mut new_col = 0;
    vec_to_rotate.iter().fold(vec![0; vec_len], |mut matrix_rotated, cur_pixel| {
        new_row = col;
        new_col = n - 1 - row;
        matrix_rotated[n * new_row + new_col] = *cur_pixel;
        col += 1;
        if col % n == 0 {
            row += 1;
            col = 0;
        }
        matrix_rotated
    })
}

/*
fn how_to_use_matrix_idx() {
    let n = 4;
    let mut row = 0;
    let mut col = 0;
    let mut new_row;
    let mut new_col;
    for _ in 0..16{
        new_row = col;
        new_col = n - 1 - row;
        println!("{}", n * new_row + new_col);
        col += 1;
        if col % n == 0 {
            row += 1;
            col = 0;
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_rotation() {
        let image = vec![
            vec![1,2,3,4],
            vec![5,6,7,8],
            vec![9,10,11,12],
            vec![13,14,15,16]
        ];
        let rotated_image = matrix_rotation(image);
        let expected_rotated_image = vec![
            vec![13,9,5,1],
            vec![14,10,6,2],
            vec![15,11,7,3],
            vec![16,12,8,4]
        ];
        assert_eq!(rotated_image, expected_rotated_image);
        let image_2 = vec![
            vec![1,2,3,4,5],
            vec![6,7,8,9,10],
            vec![11,12,13,14,15],
            vec![16,17,18,19,20],
            vec![21,22,23,24,25]
        ];
        let rotated_image_2 = matrix_rotation(image_2);
        let expected_rotated_image_2 = vec![
            vec![21,16,11,6,1],
            vec![22,17,12,7,2],
            vec![23,18,13,8,3],
            vec![24,19,14,9,4],
            vec![25,20,15,10,5]
        ];
        assert_eq!(rotated_image_2, expected_rotated_image_2);
        let empty_vec = vec![vec![]];
        // clone,copy: p.279-p.281
        let expected_empty_vec = empty_vec.clone();
        assert_eq!(matrix_rotation(empty_vec),expected_empty_vec);
        let image_not_square = vec![
            vec![1,2,3,4],
            vec![5,6,7,8]
        ];
        let expected_image_not_square = image_not_square.clone();
        assert_eq!(matrix_rotation(image_not_square), expected_image_not_square);
    }
    #[test]
    fn test_matrix_rotation_with_flatten_vec() {
        let image = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let rotated_image = matrix_rotation_with_flatten_vec(image);
        let expected_rotated_image = vec![13,9,5,1,14,10,6,2,15,11,7,3,16,12,8,4];
        assert_eq!(rotated_image, expected_rotated_image);
    }
}

fn main() {
    let rotated_image_flatten = matrix_rotation_with_flatten_vec(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
    for i in rotated_image_flatten {
        println!("{}", i);
    }
    let image = vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12],
        vec![13,14,15,16]
    ];
    matrix_rotation(image);
    //how_to_use_matrix_idx();
}
