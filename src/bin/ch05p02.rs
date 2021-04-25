// https://dangerous-animal141.hatenablog.com/entry/2014/05/10/000000
/*
2倍して1以上なら1，1未満なら0を列に追加する．2倍した値から1を引く．
*/

// 既存の関数で対応する場合．
fn float_bits_to_str(n: f64) -> String {
    let bits = n.to_bits();
    let mut ret = String::new();
    for i in 0..64 {
        if (1 << (63 - i)) & bits != 0 {
            ret.push('1');
        } else {
            ret.push('0');
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_to_string() {
        assert_eq!(
            float_bits_to_str(0.72),
            "0011111111100111000010100011110101110000101000111101011100001010"
        );
    }
}

fn main() {
    float_bits_to_str(0.72);
}
