// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut fc = HashMap::new();
    for &w in magazine {
        let mut cc = fc.entry(w).or_insert(0);
        *cc += 1;
    }
    for &w in note {
        let cc = fc.entry(w).or_insert(0);
        if *cc == 0 { return false; }
        *cc -= 1;
    }
    true
}
