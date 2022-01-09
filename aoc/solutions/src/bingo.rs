use std::str::FromStr;

#[derive(Debug)]
struct Board {
    values: Vec<Vec<u32>>,
    row_sums: Vec<u32>,
    col_sums: Vec<u32>,
}

impl FromStr for Board {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.split("\n")
            .collect();
        let values: Vec<Vec<u32>> = rows.into_iter().map(|r| r
            .trim()
            .split(" ")
            .filter(|&s| !s.trim().is_empty())
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect()
        )
            .collect();
        // dbg!(values);
        Ok(Self {
            values,
            row_sums: vec![],
            col_sums: vec![]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_test() {
        let (draws, boards) = include_str!("../inputs/day04.test")
            .trim()
            .split_once("\n\n")
            .unwrap();
        let draws: Vec<u32> = draws
            .split(",")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let boards: Vec<Board> = boards
            .split("\n\n")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        dbg!(draws);
        dbg!(boards);
        assert_eq!(3, 3);
    }
}
