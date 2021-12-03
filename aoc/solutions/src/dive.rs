fn dive(s: &str) -> u32 {
    let a = s.lines();
    let mut x:u32 = 0;
    let mut y:u32 = 0;
    for step in a {
        let mut c = step.split(' ');
        let dir = c.next().unwrap();
        let dist = c.next().unwrap_or("0").parse::<u32>().unwrap();
        // println!("dir {}, dist {}", dir, dist);
        match dir {
            "forward" => { x += dist },
            "down"    => { y += dist },
            "up"      => { y -= dist },
            _ => {}
        };
    }
    x*y
}

fn jive(s: &str) -> i32 {
    let a = s.lines();
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut aim:i32 = 0;
    for step in a {
        let mut c = step.split(' ');
        let dir = c.next().unwrap();
        let dist = c.next().unwrap_or("0").parse::<i32>().unwrap();
        match dir {
            "forward" => { x += dist; y += aim*dist; },
            "down"    => { aim += dist },
            "up"      => { aim -= dist },
            _ => {}
        };
        // println!("dir {}, dist {}, {} {} {}", dir, dist, x, y, aim);
    }
    x*y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dive_forward_5_down_5_is_25() {
        let plan = String::from("forward 5\ndown 5");
        assert_eq!(dive(&plan), 25);
    }

    #[test]
    fn dive_forward_5_down_5_up_2_is_15() {
        let plan = String::from("forward 5\ndown 5\nup 2");
        assert_eq!(dive(&plan), 15);
    }

    #[test]
    fn dive_with_sample_input_is_150() {
        let plan = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        assert_eq!(dive(&plan), 150);
    }

    #[test]
    fn dive_with_puzzle_input_is_2027977() {
        let plan = include_str!("../inputs/day2.txt");
        assert_eq!(dive(&plan), 2027977);
    }

    #[test]
    fn jive_with_sample_input_is_900() {
        let plan = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        assert_eq!(jive(&plan), 900);
    }

    #[test]
    fn jive_with_puzzle_input_is_900() {
        let plan = include_str!("../inputs/day2.txt");
        assert_eq!(jive(&plan), 900);
    }
}
