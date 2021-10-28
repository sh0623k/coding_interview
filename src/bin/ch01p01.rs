/*
   問題: ある文字列が、すべて固有である（重複する文字がない）かどうかを判定するアルゴリズムを実装する。新たなデータ構造が使えない場合についても考える。
 */

// ブロックとセミコロン → p.122
// セミコロンがついているブロック：ブロックとしての値が()になる
// expected type `()`は、セミコロンないときのエラーmsg

// p.374
extern crate coding_interview;
use coding_interview::str_helper;
use std::collections::HashSet;

// ASCIIかutf8か確認
// ASCIIであれば128より文字列が長ければfalse
// type cast : p.136
// HashMapの場合は、valueはbool
fn is_all_chars_unique(s: &str) -> bool {
    if s.len() as i32 > 128 {
        return false
    }
    let mut character_set = HashSet::new();
    for c in s.chars() {
       if character_set.contains(&c) {
           return false
       } 
       character_set.insert(c);
    }
    true
}

// テキストに対するイテレート → p.393
fn is_all_chars_unique_without_add_data_structure(s: &str) -> bool {
    if s.len() as i32 > 128 {
        return false
    }
    let mut next_char_boundary;
    for (i, char_to_compare) in s.char_indices() {
        next_char_boundary = str_helper::get_next_char_boundary(s, i);
        for c in s[next_char_boundary..].chars() {
            if char_to_compare == c {
                return false
            }
        }
    }
    true
/*
    let mut cnt;
    for char_to_compare in s.chars() {
        cnt = 0;
        // s[i..].char()がダメな場合
        for c in s.chars() {
            if char_to_compare == c {
                cnt += 1;
            }
            if cnt > 1 {
                return false
            }
        } 
    }
    true
*/
}

// 要素へのアクセス → p.354
// テキストに対するイテレート → p.393
fn is_all_chars_unique_with_sorted_str(s: &str) -> bool {
    if s.len() as i32 > 128 {
        return false
    }
    let mut char_buf = ' ';
    for (i, c) in s.chars().enumerate() {
        if i != 0 && char_buf == c {
            return false
        }
        char_buf = c;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_all_chars_unique() {
        assert_eq!(is_all_chars_unique(&String::from("abcdefg")), true);
        assert_eq!(is_all_chars_unique(&String::from("abccdefg")), false);
    }
    #[test]
    fn test_is_all_chars_unique_without_add_data_structure() {
        assert_eq!(is_all_chars_unique_without_add_data_structure(&String::from("abcdefg")), true);
        assert_eq!(is_all_chars_unique_without_add_data_structure(&String::from("abccdefg")), false);
        assert_eq!(is_all_chars_unique_without_add_data_structure(&String::from("abcdēfg")), true);
        assert_eq!(is_all_chars_unique_without_add_data_structure(&String::from("abcdēēfg")), false);
    }
    #[test]
    fn test_is_all_chars_unique_with_sorted_str() {
        assert_eq!(is_all_chars_unique_with_sorted_str(&String::from("abcdefg")), true);
        assert_eq!(is_all_chars_unique_with_sorted_str(&String::from("abccdefg")), false);
    }

    // 仕様確認
    #[test]
    #[should_panic] // p.172
    fn test_range() {
        assert_eq!(&String::from("abcdēfg")[5..], "ēfg");
    }
}

fn main () {
    is_all_chars_unique(&String::from("abcc"));
    is_all_chars_unique_without_add_data_structure(&String::from("abcc"));
    is_all_chars_unique_with_sorted_str(&String::from("abcc"));
}
