use std::convert::TryInto;

fn is_multiple_of_any(n: &u32, factors: &[u32]) -> bool { false }

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let count = (1..=limit)
        .filter(|x| is_multiple_of_any(x, factors))
        .count();
    (count as usize).try_into().unwrap()
}
