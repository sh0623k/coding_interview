// 場合の数の計算は整数型だとオーバーフローしやすい．
// 再帰の形で書いてもいい．
fn triple_step(size: usize) -> usize {
    if size < 3 {
        return size
    }
    if size == 3 {
        return 4
    }
    let mut table = vec![0; size + 1];
    table[0] = 1;
    table[1] = 1;
    table[2] = 2;
    for i in 3..size + 1 {
        table[i] = table[i - 1] + table[i - 2] + table[i - 3];
    }
    table[size]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_triple_step() {
        assert_eq!(triple_step(1), 1);
        assert_eq!(triple_step(2), 2);
        assert_eq!(triple_step(3), 4);
        assert_eq!(triple_step(4), 7);
        assert_eq!(triple_step(5), 13); 
    }
}
fn main() {
    triple_step(1);
}