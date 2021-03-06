/*
   問題: 8*8のチェス盤上に、縦・横・斜めの直線状に2つ以上並ばないように8つのクイーンを配置するすべての場合を出力するアルゴリズムを書く。（斜めはすべての斜めの線。）
*/

// 8.11と同じく，重複がないよう行や列ごとに再帰する．
// 1行にクイーンは1つのみなので，結果は1次元配列で済む．
static GRID_SIZE: usize = 8;

fn check_valid(columns: &mut Vec<usize>, row1: usize, column1: usize) -> bool {
    // take: p.326
    // 指定の数で停止する．
    for (row2, column2) in columns.iter().take(row1).enumerate() {
        if column1 == *column2 {
            return false;
        }

        let column_distance = ((*column2 as i64) - (column1 as i64)).abs() as usize;

        let row_distance = row1 - row2;
        if column_distance == row_distance {
            return false;
        }
    }
    true
}

fn place_queens(row: usize, columns: &mut Vec<usize>, results: &mut Vec<Vec<usize>>) {
    if row == GRID_SIZE {
        results.push(columns.clone());
    } else {
        for col in 0..GRID_SIZE {
            if check_valid(columns, row, col) {
                columns[row] = col;
                place_queens(row + 1, columns, results);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_queens() {
        let mut results: Vec<Vec<usize>> = vec![];
        let mut columns: Vec<usize> = vec![0; 8];
        place_queens(0, &mut columns, &mut results);
        assert_eq!(results.len(), 92);
    }
}

fn main() {
    let mut results: Vec<Vec<usize>> = vec![];
    let mut columns: Vec<usize> = vec![0; 8];
    place_queens(0, &mut columns, &mut results);
}
