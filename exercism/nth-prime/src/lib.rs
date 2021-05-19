
fn is_prime(p:& u32) -> bool {
    (*p < 3) || {
        let l = ((*p as f64).sqrt()) as u32;
        !(2..=l).any(|i| (*p)%i == 0)
    }
}
pub fn nth(n: u32) -> u32 {
    (2u32..).filter(is_prime).nth(n as usize).unwrap()
}
