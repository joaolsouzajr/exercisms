
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    reverse_unicode(input)
}

// First solution for UTF-8
pub fn reverse_utf8(input: &str) -> String {
    let chars: Vec<&str> = input.rsplit("").collect();
    chars.iter().as_slice().join("")
}

// Best solution for UTF-8
pub fn reverse_utf8_best_solution(input: &str) -> String {
    input.chars().rev().collect()
}

// First solution for Unicode
pub fn reverse_unicode(input: &str) -> String {
    let mut chars: Vec<&str> = input.graphemes(true).collect();
    chars.reverse();
    chars.iter().as_slice().join("")
}

// Best solution for Unicode
pub fn reverse_unicode_best_solution(input: &str) -> String {
    input.graphemes(true).rev().collect()
}