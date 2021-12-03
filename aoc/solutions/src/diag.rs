
fn mfb(s: &str, pos: usize) -> char {
    let (num_ones, num_zeroes) = count_ones_and_zeros(s, pos);
    
    if num_ones > num_zeroes {
        return '1';
    } else {
        return '0';
    }
}

fn count_ones_and_zeros(s: &str, pos: usize) -> (i32, i32) {
    let num_ones= s.lines().filter(|line| line.chars().nth(pos) == Some('1')).count() as i32 ;
    let num_zeroes= s.lines().filter(|line| line.chars().nth(pos) == Some('0')).count() as i32 ;
    (num_ones, num_zeroes)
}

fn get_nums(s: &[&str], pos: usize) -> (i32, i32) {
    let num_ones= s.iter().filter(|line| line.chars().nth(pos) == Some('1')).count() as i32 ;
    let num_zeroes= s.iter().filter(|line| line.chars().nth(pos) == Some('0')).count() as i32 ;
    (num_ones, num_zeroes)
}

fn diag(s: &str) -> u32 {
    let bits = s.lines().map(|line| line.len()).max().unwrap();
    let inp = s.lines().collect::<Vec<_>>();
    let mut gfs = String::new();
    let mut efs = String::new();
    for i in 0..bits {
        let (o,z) = get_nums(&inp, i);
        gfs.push(if o>z {'1'} else { '0' });
        efs.push(if o>z {'0'} else { '1' });
    }
    let gf = u32::from_str_radix(&gfs, 2).unwrap();
    let ef = u32::from_str_radix(&efs, 2).unwrap();
    return gf * ef;
}

fn life_support(s: &str) -> u32 {
    let bits = s.lines().map(|line| line.len()).max().unwrap();
    let mut o2v = s.lines().collect::<Vec<_>>();
    for i in 0..bits {
        let ov = o2v.iter().filter(|line| line.chars().nth(i) == Some('1')).map(|x| *x).collect::<Vec<_>>();
        let zv = o2v.iter().filter(|line| line.chars().nth(i) == Some('0')).map(|x| *x).collect::<Vec<_>>();
        o2v = if ov.len() >= zv.len() { ov } else { zv };
        if o2v.len() < 2 { break; }
    }
    // println!("O2 is {:?}", o2v);
    let mut co2 = s.lines().collect::<Vec<_>>();
    for i in 0..bits {
        let ov = co2.iter().filter(|line| line.chars().nth(i) == Some('1')).map(|x| *x).collect::<Vec<_>>();
        let zv = co2.iter().filter(|line| line.chars().nth(i) == Some('0')).map(|x| *x).collect::<Vec<_>>();
        co2 = if zv.len() <= ov.len() { zv } else { ov };
        if co2.len() < 2 { break; }
    }
    // println!("CO2 is {:?}", co2);
    let o2f = u32::from_str_radix(&o2v.first().unwrap(), 2).unwrap();
    let cof = u32::from_str_radix(&co2.first().unwrap(), 2).unwrap();
    return o2f * cof;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diag_with_sample_input_is_198() {
        let readings = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(diag(&readings), 198);
    }

    #[test]
    fn diag_with_puzzle_input_is_the_answer() {
        let readings = include_str!("../inputs/day3.txt");
        assert_eq!(diag(&readings), 3148794);
    }

    #[test]
    fn life_support_with_sample_input_is_230() {
        let readings = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(life_support(&readings), 230);
    }

    #[test]
    fn life_support_with_puzzle_input_is_the_answer() {
        let readings = include_str!("../inputs/day3.txt");
        assert_eq!(life_support(&readings), 2795310);
    }
}
