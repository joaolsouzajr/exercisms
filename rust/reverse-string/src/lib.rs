// use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // let mut chars: Vec<&str> = input.graphemes(true).collect();
    // chars.reverse();
    // chars.iter().as_slice().join("")
    input.chars().rev().collect()
}




