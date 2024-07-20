pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    if digits.len() < len {
        return list;
    }
    let mut left = 0;
    let mut right = len - 1;
    while right < digits.len() {
        list.push(digits[left..=right].to_string());
        left += 1;
        right = left + len - 1;
    }
    list
}