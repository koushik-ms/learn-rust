pub fn factors(n: u64) -> Vec<u64> {
    let mut ans = Vec::new();
    let mut m = n;
    for i in 2.. {
        if i > m { break; }
        while m%i == 0 {
            ans.push(i);
            m = m/i;
        }
    }
    ans
}
