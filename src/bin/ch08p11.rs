// 金種ごとにループすることで，重複を回避できる．
// 組み合わせになってほしくないときは，重複を回避するようなループを組む．

fn make_change(amt: usize) -> usize {
    let coins: [usize; 4] = [25, 10, 5, 1];
    let mut table = vec![vec![0; amt + 1]; coins.len()];
    make_change_with_coins(amt, coins, 0, &mut table)
}

fn make_change_with_coins(amt: usize, coins: [usize; 4], coins_idx: usize, table: &mut Vec<Vec<usize>>) -> usize {
    if coins_idx >= coins.len() - 1 {
        return 1
    }
    if table[coins_idx][amt] > 0 {
        return table[coins_idx][amt]
    }
    let mut ways = 0;
    let coin_amt = coins[coins_idx];
    let mut cnt = 0;
    while cnt * coin_amt <= amt {
        // tableは&mutなので，そのまま渡せばよい．
        ways += make_change_with_coins(amt - cnt * coin_amt, coins, coins_idx + 1, table);
        cnt += 1;
    }
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_change() {
        assert_eq!(make_change(10), 4);
        assert_eq!(make_change(11), 4);
        assert_eq!(make_change(15), 6);
    }
}

fn main() {
    make_change(10);
}
