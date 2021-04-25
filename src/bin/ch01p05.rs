// match: p.126
fn is_convertible_by_once(str_1: &str, str_2: &str) -> bool {
    // is_a_char_missingを呼ぶときに2つのstrの長さチェックする手もある。
    // キャラカウントの差は符号あり整数にキャストしてabs()
    let long_str;
    let short_str;
    if str_1.len() > str_2.len() {
        long_str = &str_1;
        short_str = &str_2;
    }
    else {
        long_str = &str_2;
        short_str = &str_1;
    }
    // match: p.126
    match long_str.chars().count() - short_str.chars().count() {
        0 => is_a_char_different(long_str, short_str),
        1 => is_a_char_missing(long_str, short_str),
        _ => false
    }
}

fn is_a_char_different(str_1: &str, str_2: &str) -> bool {
    let mut found_different_char = false;
    // zip: p.332
    for (char_1, char_2) in str_1.chars().zip(str_2.chars()){
        if char_1 != char_2 {
            if !found_different_char {
                found_different_char = true
            } else {
                return false
            }
        }
    }
    true
    /*
    let mut itr_char_1 = str_1.chars();
    let mut char_1;
    for char_2 in str_2.chars() {
        char_1 = itr_char_1.next();
        if !found_different_char {
            if char_1 != None && char_2 != char_1.unwrap() {
                found_different_char = true;
            }
        }
        else{
            if char_1 != None && char_2 != char_1.unwrap() {
                return false
            }
        }
    } 
    true
    */
}

// Stringとstr: p.387
// サイズ可変なバッファが必要か、すでにあるテキストをそのまま使えるか
// https://doc.rust-lang.org/std/str/index.html
fn is_a_char_missing(long_str: &str, short_str: &str) -> bool{
    // 長さチェックが必要。
    if long_str.chars().count() <= short_str.chars().count() {
        return false
    }
    let mut found_missing_char = false;
    let mut itr_char_long = long_str.chars();
    let mut char_long;
    for char_short in short_str.chars() {
        // Option: p.98
        // https://doc.rust-lang.org/std/option/enum.Option.html
        char_long = itr_char_long.next();
        if !found_missing_char {
            if char_long != None && char_short != char_long.unwrap() {
                found_missing_char = true;
                char_long = itr_char_long.next();
            }
        }
        if char_long != None && char_short != char_long.unwrap() {
            return false
        }
    } 
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_convertible_by_once_brute() {
        assert_eq!(is_convertible_by_once(&String::from("pale"), &String::from("ple")), true);
        assert_eq!(is_convertible_by_once(&String::from("pales"), &String::from("pale")), true);
        assert_eq!(is_convertible_by_once(&String::from("pale"), &String::from("bale")), true);
        assert_eq!(is_convertible_by_once(&String::from("pale"), &String::from("bake")), false);
    }
}

fn main() {
    is_convertible_by_once(&String::from("pale"), &String::from("ple"));
}