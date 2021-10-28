/*
   問題: 文字列内に出現するすべての空白文字を'%20'で置き換えるメソッドを書く。文字列の後ろには新たに文字を追加するためのスペースが十分にあることと、その追加用スペースを除いた文字列の真の長さが与えられる。
*/

// p.391-p.393
fn urlify_drain(str_to_url: &mut String, length: usize) -> String {
    str_to_url.drain(length..);
    str_to_url.replace(" ", "%20")
}
// trim_right() p.395
// trim_end()にしたほうがよいらしい
// 末尾にスペースなしの想定
fn urlify(str_to_url: &str) -> String {
    str_to_url.trim_end().replace(" ", "%20")
}

// 末尾にスペースなしの想定
// fold: p.340, 畳み込み
// acc: accumulate:「蓄積する」。ad(方向)とcumulare(積み上げる)
fn urlify_fold(str_to_url: &str) -> String {
    str_to_url.split_whitespace().fold(String::new(), |acc, s| {
        if acc.is_empty() {
            String::from(s)
        }
        else {
            acc + "%20" + s
        }
    })
}

/*
インデックス操作不可
popとpushで逆に生成して最後にひっくり返せばまあまあなものにはなりそう
fn urlify_without_replace(str_to_url: &mut String, length: usize) -> String {
    if str_to_url.bytes().count() != str_to_url.chars().count() {
        return String::from("Just for ASCII characters.")
    }
    let mut spaces_count = 0;
    for c in str_to_url.chars() {
        if c == ' ' {
            spaces_count += 1;
        }
    }
    let mut idx = length - 1 + spaces_count * 2;
    while idx >= 0 {
        // インデックス操作が入る
    }
    str_to_url.to_string()
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_urlify() {
        assert_eq!(urlify(&mut String::from("Mr John Smith ")), "Mr%20John%20Smith");
        assert_eq!(urlify_drain(&mut String::from("Mr John Smith "), 13), "Mr%20John%20Smith");
        assert_eq!(urlify_fold(&mut String::from("Mr John Smith ")), "Mr%20John%20Smith");
    }
}
fn main() {
    urlify(&mut String::from("Mr John Smith "));
    urlify_drain(&mut String::from("Mr John Smith "), 13);
    urlify_fold(&mut String::from("Mr John Smith "));
}
