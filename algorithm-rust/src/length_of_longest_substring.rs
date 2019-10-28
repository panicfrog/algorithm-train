#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut chars: Vec<char> = Vec::new();
    let mut lenth = 0;
    for c in s.chars() {
        if let Some(i) = chars.iter().
            position(|v| *v == c) {
            chars = chars[i+1..].to_vec();
        }
        chars.push(c);
        lenth = if chars.len() > lenth { chars.len() } else { lenth };
    }
    lenth as i32
}