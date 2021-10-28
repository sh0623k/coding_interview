/*
    問題: モノクロのスクリーンが1次元のバイト型配列として保持されている。1バイトには連続した8ピクセルを保持することができる。スクリーンの幅は8の倍数で、バイトの途中で切れるような形にはなっていないことにする。(x1, y)から(x2, y)まで水平な直線を描く関数を実装する。メソッドはdrowLine(byte[] screen, int width, int x1, int x2, int y)とする。
*/

// あるビットを1に更新するときは，|= 1 << (i)でいい．
// OSを作る時と同じ感じ．
// 間のバイトは0xFFで埋めたほうが速い．start, endはマスクを作る．
fn draw_line(screen: &mut Vec<u8>, width: usize, x1: usize, x2: usize, y: usize) {
    let start = x1 + y * width;
    let end = x2 + y * width;
    for i in start..end {
        screen[i / 8] |= 1 << (i % 8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_line() {
        let v: u8 = 0;
        let mut vec = vec![v; 8];
        draw_line(&mut vec, 8, 0, 8, 1);
        assert_eq!(vec, vec![0, 255, 0, 0, 0, 0, 0, 0]);
    }
}

fn main() {
    let v: u8 = 0;
    let mut vec = vec![v; 8];
    draw_line(&mut vec, 8, 0, 8, 1);
}
