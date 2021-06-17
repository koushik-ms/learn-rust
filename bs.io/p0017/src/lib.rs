use std::u32;

const BOUND: u32 = 1_000_000_000 + 7;

#[allow(dead_code)]
fn traverse(mat: &Vec<Vec<u8>>) -> u32 { 
    let n = mat.len();
    let m = mat.first().unwrap().len();
    let mut ways = vec![vec![0;m];n];
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 1 { continue; }
            if i==0 && j==0 {
                ways[i][j] = 1;
            } else {
                ways[i][j] = {
                    let fu = if 0==i {0} else {ways[i-1][j]};
                    let fl = if 0==j {0} else {ways[i][j-1]};
                    (fu+fl)%BOUND
                };
            }
        }
    }
    ways[n-1][m-1]
 }
#[cfg(test)]
mod tests {
    use super::traverse;
    fn maze(rep: &str) -> Vec<Vec<u8>> {
        let rows = rep.split(',');
        rows.map(|r| {
            r.bytes().map(|c| c - '0' as u8).collect::<Vec<_>>()
        }).collect()
    }
    #[test]
    fn traverse_001_001_100_in_2_ways() {
        let m = maze("001,001,100");
        println!("{:?}", m);
        assert_eq!(traverse(&m), 2);
    }
}
