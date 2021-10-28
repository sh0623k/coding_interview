/*
   問題: ある整数AからBに変換するのに必要なビット数を決定する関数を書く。
*/

/*
単純にXORをとればOK．
ある数cの最下位の1を反転するには，c&(c-1)をとればよい．
*/

fn get_bit_swap_required_count(a: i32, b: i32) -> usize {
    let mut cnt = 0;
    let mut c = a ^ b;
    while c != 0 {
        c = c & (c - 1);
        cnt += 1;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(get_bit_swap_required_count(0b1_1101, 0b0_1111), 2);
    }
}

fn main() {
    get_bit_swap_required_count(0b1_1101, 0b0_1111);
}
