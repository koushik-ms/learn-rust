use std::convert::TryInto;

fn is_multiple_of_any(n: &u32, factors: &[u32]) -> bool { 
    factors.iter().any(|&f| n%f == 0)
 }

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..=limit)
        .filter(|x| is_multiple_of_any(x, factors))
        .sum()
}
