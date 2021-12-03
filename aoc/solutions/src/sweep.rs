fn sweep(measurements: &[u32]) -> usize {
    measurements
        .windows(2)
        .fold(0, |acc, win| if win[0] < win[1] { acc + 1 } else { acc })
}

fn tokenize_input(input: &str) -> Vec<u32> {
    let mut res = vec![];

    for line in input.lines() {
        res.push(line.parse::<u32>().unwrap());
    }

    res
}

fn slide(measurements: &[u32]) -> usize {
    let w: Vec<_> = measurements
        .windows(3)
        .map(|x| x.iter().sum::<u32>())
        .collect();
    sweep(&w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_detects_7_increases() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(sweep(&measurements), 7);
    }

    #[test]
    fn read_input_to_vec() {
        let s = include_str!("../inputs/day1.txt");
        let vec_from_file = tokenize_input(&s);
        assert_eq!(vec_from_file.len(), 2000);
    }

    #[test]
    fn sweep_detects_n_increases() {
        let s = include_str!("../inputs/day1.txt");
        let measurements = tokenize_input(&s);
        assert_eq!(sweep(&measurements), 1557);
    }

    #[test]
    fn slide_detects_5_increases() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(slide(&measurements), 5);
    }

    #[test]
    fn slide_detects_n_increases() {
        let s = include_str!("../inputs/day1.txt");
        let measurements = tokenize_input(&s);
        assert_eq!(slide(&measurements), 1608);
    }
}
