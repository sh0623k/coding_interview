/*
   問題: 片方の文字列が、もう片方の文字列の一部分になっているかどうかを調べるメソッド「isSubstring」が使えると仮定する。2つの文字列s1, s2が与えられたとき、inSubstringメソッドを1度だけ使ってs2がs1を回転させたものかどうかを判定するコードを書く。
*/

// 例を考えていって理解した。
// 分け目でそれぞれ別の変数にしても分かるようだ。
fn is_strs_rotation(str_1: &str, str_2: &str) -> bool {
    if str_1.len() != str_2.len() {
        return false
    }
    // to_owned(): p.291
    let str_1_to_str_1 = str_1.to_owned() + str_1;
    // isSubstring()の代わり
    if str_1_to_str_1.contains(str_2) {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_strs_rotation() {
        assert_eq!(is_strs_rotation(&String::from("abc"), &String::from("cab")), true);
        assert_eq!(is_strs_rotation(&String::from("abc"), &String::from("bac")), false);
        assert_eq!(is_strs_rotation(&String::from("abcdebcade"), &String::from("deabcdebca")), true);
        assert_eq!(is_strs_rotation(&String::from("abcdebcadea"), &String::from("debabcdebca")), false);
    }
}

fn main() {
    is_strs_rotation(&String::from("abcdebcade"), &String::from("deabcdebca"));
}
