use std::{num::NonZeroUsize, u32};

const BOUND: u32 = 1_000_000_000 + 7;

#[allow(dead_code)]
fn traverse(mat: &Vec<Vec<u8>>) -> u32 {
    let n = mat.len();
    let m = mat.first().unwrap().len();
    let mut ways = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 1 {
                continue;
            }
            ways[i][j] = if top_left(i, j) {
                1
            } else {
                bounded_ways_from_neighbours(&ways, i, j)
            }
        }
    }
    ways[n - 1][m - 1]
}

fn bounded_ways_from_neighbours(ways: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    (
        NonZeroUsize::new(i).map_or(0, |_| ways[i - 1][j])
        + NonZeroUsize::new(j).map_or(0, |_| ways[i][j - 1])
    ) % BOUND
}

fn top_left(i: usize, j: usize) -> bool {
    i == 0 && j == 0
}
#[cfg(test)]
mod tests {
    use super::traverse;
    fn maze(rep: &str) -> Vec<Vec<u8>> {
        let rows = rep.split(',');
        rows.map(|r| {
            r.bytes()
                .map(|c| c - '0' as u8)
                .collect::<Vec<_>>()
        }).collect()
    }
    #[test]
    fn traverse_001_001_100_in_2_ways() {
        let m = maze("001,001,100");
        println!("{:?}", m);
        assert_eq!(traverse(&m), 2);
    }
    #[test]
    fn traverse_0001_0001_1000_in_5_ways() {
        let m = maze("0001,0001,1000");
        println!("{:?}", m);
        assert_eq!(traverse(&m), 5);
    }
}
