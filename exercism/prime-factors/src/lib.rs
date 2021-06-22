pub fn factors(mut n: u64) -> Vec<u64> {
    let mut ans = Vec::new();
    for i in 2.. {
        if i > n { break; }
        while n%i == 0 {
            ans.push(i);
            n /= i;
        }
    }
    ans
}
