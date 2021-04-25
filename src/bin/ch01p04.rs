extern crate coding_interview;
use coding_interview::str_helper;

// ビット演算にするとより良い。
fn is_palindrome(s: &str) -> bool {
//  boolにしてtrueになっていたらreturn falseの方が良い
//  let mut allowed_odd_char_cnt = 1;
    let mut found_odd = false;
    let normalized_str = s
        .to_lowercase()
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s);
    let char_map = str_helper::create_char_count_map_from_str(&normalized_str);
    for (_, char_cnt) in char_map {
        if char_cnt % 2 > 0 {
            if found_odd {
                return false
            }
            found_odd = true;
        }
//      boolにしてtrueになっていたらreturn falseの方が良い
//      allowed_odd_char_cnt -= (char_cnt % 2) as isize;
//      if allowed_odd_char_cnt < 0 {
//          return false
//      }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(&String::from("Tact Coa")), true);
        assert_eq!(is_palindrome(&String::from(" Tact Coa  ")), true);
        assert_eq!(is_palindrome(&String::from("aabbcc")), true);
        assert_eq!(is_palindrome(&String::from("aabbēē")), true);
        assert_eq!(is_palindrome(&String::from("aaacccc")), true);
        assert_eq!(is_palindrome(&String::from("abbc")), false);
        assert_eq!(is_palindrome(&String::from("aaabbccc")), false);
    }
}

fn main() {
    is_palindrome(&String::from("Tact Coa"));
}