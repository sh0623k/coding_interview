// ビット演算: p.135

/*
クリア→シフト→マージ
*/
// ビットの更新は，一度更新対象のビットのみ0であるマスクとの&をとって，その後更新したいビットとの|をとる．
// ある桁のみ1にしたい場合は，1 << digitとの|をとる．複数の桁で1にしたい場合は，その分だけ繰り返す．
fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
    let mut mask = 0;
    for cnt in i..j+1 {
        mask = mask | (1 << cnt);
    }
    (n & !mask) | m << i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_bits() {
        assert_eq!(insert_bits(0b100_0000_0000, 0b10011, 2, 6), 0b100_0100_1100);
    }
}

fn main() {
    insert_bits(0b100_0000_0000, 0b10011, 2, 6);
}