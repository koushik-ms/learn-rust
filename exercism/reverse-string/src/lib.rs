extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect()
    // input.rsplit("").collect()
    input.graphemes(true).rev().collect()
}
