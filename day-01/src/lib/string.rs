// String searching routines


/// Find the first
#[must_use]
pub fn find_of_many(input: &str, matches: &[&str]) -> Option<usize> {
    let length = input.len(); // Lenght of string in bytes

    for start in 0..length {
        if !input.is_char_boundary(start) {
            continue
        }
        for (idx, name) in matches.iter().enumerate() {
            if input[start..length].starts_with(*name) {
                return Some(idx)
            }
        }
    }
    None
}

/// Find the last
#[must_use]
pub fn rfind_of_many(input: &str, matches: &[&str]) -> Option<usize> {
    let length = input.len(); // Lenght of string in bytes

    for start in (0..length).rev() {
        if !input.is_char_boundary(start) {
            continue
        }
        for (idx, name) in matches.iter().enumerate() {
            if input[start..length].starts_with(*name) {
                return Some(idx)
            }
        }
    }
    None
}
