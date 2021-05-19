
fn is_prime(p: u32) -> bool {
    if p < 3 {return true;}
    for i in 2..=(p/2) {
        if p%i == 0 {return false;}
    }
    true
}

fn nxt_prime(x: &mut u32) -> u32 {
    *x += 1;
    loop {
        if is_prime(*x) { break; }
        *x += 1;
    }
    *x
}

pub fn nth(n: u32) -> u32 {
    let mut ans: u32 = 1;
    std::iter::from_fn(|| {
        Some(nxt_prime(&mut ans))
    }).nth(n as usize).unwrap()
}
