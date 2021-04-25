use std::collections::HashMap;

// entry : p.373
pub fn create_char_count_map_from_str(s: &str) -> HashMap<char, usize> {
    let mut char_count_map = HashMap::new();
    for c in s.chars() {
        let cnt = char_count_map.entry(c).or_insert(0);
        *cnt += 1;
    }
    char_count_map
}

pub fn get_next_char_boundary(s: &str, cur_offset: usize) -> usize {
    let mut next_char_boundary;
    next_char_boundary = cur_offset + 1;
    // p.389
    while !s.is_char_boundary(next_char_boundary) {
        next_char_boundary += 1;
    }
    next_char_boundary
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_char_count_map_from_str() {
        let char_count_map = create_char_count_map_from_str(&String::from("abddceee"));
        // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
        assert_eq!(char_count_map[&'a'], 1);
        assert_eq!(char_count_map[&'d'], 2);
        assert_eq!(char_count_map[&'e'], 3);
        // p.370
        assert_eq!(char_count_map.contains_key(&'f'), false);
    }
    #[test]
    fn test_get_next_char_boundary() {
        let s = &String::from("abcdēfg");
        assert_eq!(get_next_char_boundary(s, 4), 6);
        assert_eq!(&s[get_next_char_boundary(s, 3)..], "ēfg");
        assert_eq!(&s[get_next_char_boundary(s, 4)..], "fg");
    }
}