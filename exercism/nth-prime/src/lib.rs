
fn is_prime(p: u32) -> bool {
    if p < 3 {return true;}
    for i in 2..=(p/2) {
        if p%i == 0 {return false;}
    }
    true
}

fn nxt_prime(x: &mut u32) {
    let mut p = *x+1;
    loop {
        if is_prime(p) { break; }
        p += 1;
    }
    *x=p
}
pub fn nth(n: u32) -> u32 {
    let mut ans: u32 = 1;
    let mut ps = std::iter::from_fn(|| {
        nxt_prime(&mut ans);
        Some(ans)
    });
    ps.nth(n as usize).unwrap()
}

