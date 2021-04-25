fn str_compression_with_fold(s: &str) -> String {
    let mut cnt = 0;
    let mut str_compressed = s.chars().fold(String::new(), |mut acc, current_char| {
        if acc.is_empty() {
            acc.push(current_char);
            cnt = 1;
        } else if acc.chars().last().unwrap() == current_char {
            cnt += 1;
        } else {
            acc.push_str(&format!("{}", cnt));
            acc.push(current_char);
            cnt = 1;
        }

        acc
    });
    str_compressed.push_str(&format!("{}", cnt));

    if s.len() <= str_compressed.len() {
        String::from(s)
    } else {
        str_compressed
    }
}

fn str_compression(s: &str) -> String {
    let mut str_compressed = String::new();
    let mut char_buf = ' '; // 最初のループは問答無用で上書きするから通るとは思うが初期化は？
    let mut char_cnt = 0;
    for c in s.chars() {
        if str_compressed.is_empty() {
            char_buf = c;
            str_compressed.push(c);
            char_cnt = 1;
        } else if c == char_buf {
            char_cnt += 1;
        } else {
            // push: p.389
            // https://qiita.com/smicle/items/29a4d5d1d14ad7f77f60
            str_compressed.push_str(&char_cnt.to_string());
            char_buf = c;
            str_compressed.push(c);
            char_cnt = 1;
        }
    }
    str_compressed.push_str(&char_cnt.to_string());
    if str_compressed.len() < s.len() {
        str_compressed
    }
    else {
        String::from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_compression_with_fold() {
        assert_eq!(str_compression_with_fold(&String::from("aabcccccaaa")), String::from("a2b1c5a3"));
        assert_eq!(str_compression_with_fold(&String::from("abcd")), String::from("abcd"));
        assert_eq!(str_compression_with_fold(&String::from("ss")), String::from("ss"));
        assert_eq!(str_compression_with_fold(&String::from("ssssssssss")), String::from("s10"));
        assert_eq!(str_compression_with_fold(&String::from("")), String::from(""));
    }
    #[test]
    fn test_str_compression() {
        assert_eq!(str_compression(&String::from("aabcccccaaa")), String::from("a2b1c5a3"));
        assert_eq!(str_compression(&String::from("abcd")), String::from("abcd"));
        assert_eq!(str_compression(&String::from("ss")), String::from("ss"));
        assert_eq!(str_compression(&String::from("ssssssssss")), String::from("s10"));
        assert_eq!(str_compression(&String::from("")), String::from(""));
    }
}

fn main() {
    str_compression_with_fold(&String::from("aabcccccaaa"));    
    str_compression(&String::from("aabcccccaaa"));    
}