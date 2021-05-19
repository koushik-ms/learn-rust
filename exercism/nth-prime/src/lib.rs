
fn is_prime(p: u32) -> bool {
    if p < 3 {return true;}
    for i in 2..=(p/2) {
        if p%i == 0 {return false;}
    }
    true
}

fn nxt_prime(x:u32) -> u32 {
    let mut p = x+1;
    loop {
        if is_prime(p) { break; }
        p += 1;
    }
    p
}
pub fn nth(n: u32) -> u32 {
    let mut ans: u32 = 1;
    for _ in 0..=n {
        ans = nxt_prime(ans);
    }
    ans
}

