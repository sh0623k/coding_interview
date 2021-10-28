/*
    問題: 2つの文字列が与えられたとき、片方がもう片方の並び替えになっているかどうかを決定するメソッドを書く。
 */

// tomlファイルを再保存しないと読み込んでくれなかった？
// src/binの扱い → p.168
extern crate coding_interview;
use coding_interview::str_helper::*;

// 大小文字や空白を確認する
// ソートして同じか確認する手もあるが、遅い
// 1つ目でinc、2つ目でdecがベストだった
fn are_two_strs_in_the_same_combination(str_a: &str, str_b: &str) -> bool {
    if str_a.len() != str_b.len() {
        return false
    }
    let char_count_map_a = create_char_count_map_from_str(str_a);
    let char_count_map_b = create_char_count_map_from_str(str_b);
    if char_count_map_a != char_count_map_b {
        return false
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_are_two_strs_in_the_same_combination() {
        assert_eq!(are_two_strs_in_the_same_combination(&String::from("A cdēe"), &String::from("A cdēe")), true);
        assert_eq!(are_two_strs_in_the_same_combination(&String::from("abcdee"), &String::from("abcdeea")), false);
        assert_eq!(are_two_strs_in_the_same_combination(&String::from("abcdee"), &String::from("awcdee")), false);
        assert_eq!(are_two_strs_in_the_same_combination(&String::from("abcdeē"), &String::from("abcdee")), false);
        assert_eq!(are_two_strs_in_the_same_combination(&String::from("Abcde"), &String::from("abcde")), false);
        assert_eq!(are_two_strs_in_the_same_combination(&String::from("abcde "), &String::from("abcde")), false);
    }
}

fn main(){
    are_two_strs_in_the_same_combination(&String::from("abcde"), &String::from("bdeca"));
}
