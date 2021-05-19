
fn is_prime(p: u32) -> bool {
    if p < 3 {return true;}
    let l = ((p as f64).sqrt()) as u32;
    for i in 2..=l {
        if p%i == 0 {return false;}
    }
    true
}

fn nxt_prime(x: &mut u32) -> u32 {
    *x += 1;
    while !is_prime(*x) { *x += 1; }
    *x
}

pub fn nth(n: u32) -> u32 {
    let mut ans: u32 = 1;
    std::iter::from_fn(|| {
        Some(nxt_prime(&mut ans))
    }).nth(n as usize).unwrap()
}
