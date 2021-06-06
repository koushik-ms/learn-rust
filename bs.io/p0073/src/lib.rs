pub fn solve(mat: Vec<Vec<u8>>) -> Vec<Vec<u8>> { mat }
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn make_board(rep: &str) -> Vec<Vec<u8>> {
        let rb = rep.as_bytes();
        let mut board = vec![];
        for i in 0..9 {
            let mut r = vec![];
            for j in 0..9 {
                r.push(
                    (rb[i*9+j] as u8) - ('0' as u8)
                );
            }
            board.push(r);
        }
        board
    }
    #[test]
    fn can_solve_puzzle() {
        let start = "004509000500100048090020560000030016340010025720060000059040070270001003000702600";
        let expected = "164589237532176948897423561985234716346917825721865394659348172278691453413752689";

        // Prepare board
        let board = make_board(start);
        println!("{:?}", board);
        assert_eq!(solve(board), make_board(expected));
    }
}
