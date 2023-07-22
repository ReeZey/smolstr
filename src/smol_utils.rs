pub fn char_to_index(charset: &String, searced_str: char) -> Option<usize> {
    for (index, char) in charset.chars().enumerate() {
        if searced_str == char {
            return Some(index);
        }
    }
    return None;
}

pub fn index_to_char(charset: &String, index: usize) -> Option<char> {
    for (i, char) in charset.chars().enumerate() {
        if i == index {
            return Some(char);
        }
    }
    return None;
}