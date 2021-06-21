use std::iter::successors;

const BOUND: u32 = 1_000_000_000 + 7;

#[allow(dead_code)]
fn solve(n:usize) -> u32 {
    successors(
        Some((1u32,1u32)), 
        |&(b, a)| Some(((b+a)%BOUND, b))
    ).nth(n-1).unwrap().0
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn climb_1_step_in_1_way() {
        assert_eq!(solve(1), 1);
    }
    #[test]
    fn climb_2_steps_in_2_ways() {
        assert_eq!(solve(2), 2);
    }
    #[test]
    fn climb_3_steps_in_3_ways() {
        assert_eq!(solve(3), 3);
    }
    #[test]
    fn climb_4_steps_in_5_ways() {
        assert_eq!(solve(4), 5);
    }
    #[test]
    fn climb_5_steps_in_8_ways() {
        assert_eq!(solve(5), 8);
    }
    #[test]
    fn climb_6_steps_in_13_ways() {
        assert_eq!(solve(6), 13);
    }
    #[test]
    fn climb_417_steps_in_430965194_ways() {
        assert_eq!(solve(417), 430965194);
    }
}
