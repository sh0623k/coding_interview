/*
   問題: 文字列の配列を、アナグラムになっている文字列がお互い隣り合うように並び変えるメソッドを書く。
*/

/*
バケットソート
隣接させるグループごとに一意のキーを作ってマップを作る．
マップのキーごとに結果リストに格納する．
*/

use std::collections::HashMap;

fn sort_chars(s: &String) -> String {
    // collect: p.342
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

fn group_anagrams(arr: Vec<String>) -> Vec<String> {
    // map: p.372
    let mut map = HashMap::<String, Vec<String>>::new();

    for s in arr {
        let key = sort_chars(&s);
        let list = map.entry(key).or_insert(vec![]);
        list.push(s);
    }

    let mut result = vec![];
    // p.373
    // map.values()でもok．
    for (_, list) in map {
        for s in list {
            println!("{}", s);
            result.push(s);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_anagrams() {
        let strs = vec![String::from("abc")
            , String::from("def")
            , String::from("ghi")
            , String::from("bac")
            , String::from("ihg")
            , String::from("efd")
        ];
        let grouped_list = group_anagrams(strs);
        let mut pre_s: Option<String> = None;
        for s in grouped_list {
            if pre_s.is_none() {
                pre_s = Some(s);
            }
            else {
                let pre_s_unwraped = pre_s.unwrap();
                // 文字列の扱い: p.392
                if pre_s_unwraped.contains("abc") {
                    assert_eq!(s, String::from("bac"));
                }
                else if pre_s_unwraped.contains("def") {
                    assert_eq!(s, String::from("efd"));
                }
                else if pre_s_unwraped.contains("ghi") {
                    assert_eq!(s, String::from("ihg"));
                }
                pre_s = None;
            }
        }
    }
}
fn main() {
    let strs = vec![String::from("abc")
        , String::from("def")
        , String::from("ghi")
        , String::from("bac")
        , String::from("ihg")
        , String::from("efd")
    ];
    group_anagrams(strs);
}
