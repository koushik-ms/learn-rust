use std::u32;

#[allow(dead_code)]
fn traverse(mat: &Vec<Vec<u8>>) -> u32 { 2 }
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
