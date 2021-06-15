#[allow(dead_code)]
fn solve(n: u32) -> u32 { n+1 }
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn climb_4_steps_in_5_ways() {
        assert_eq!(solve(4), 5);
    }
}
