fn ss(mat: &mut Vec<Vec<u8>>, ro: & mut Vec<Vec<bool>>, co: & mut Vec<Vec<bool>>, ce: & mut Vec<Vec<bool>>, pos: usize) -> bool {
    if pos>80 { return true; }
    let si = pos/9;
    let sj = pos%9;
    let sc = (si/3)*3 + (sj/3);
    match mat[si][sj] {
        0 => {
            for i in 1..10 {
                if !ro[si][i] && !co[sj][i] && !ce[sc][i] {
                    ro[si][i] = true;
                    co[sj][i] = true; 
                    ce[sc][i] = true;
                    mat[si][sj] = i as u8;
                    if ss(mat, ro, co, ce, pos+1) { return true; }
                    mat[si][sj] = 0;
                    ce[sc][i] = false;
                    co[sj][i] = false; 
                    ro[si][i] = false;
                }
            }
            false
        },
        _ => ss(mat, ro, co, ce, pos+1)
    }
}

pub fn solve(mut mat: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut ro = vec![vec![false; 10]; 9];
    let mut co = vec![vec![false; 10]; 9];
    let mut ce = vec![vec![false; 10]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let c = (i/3)*3 + (j/3);
            let v = mat[i][j] as usize;
            ro[i][v] = true;
            co[j][v] = true;
            ce[c][v] = true;
        }
    }
    let _ = ss(&mut mat, & mut ro, & mut co, & mut ce, 0);
    mat
}
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
        println!("{:?}", board[0][0]);
        assert_eq!(solve(board), make_board(expected));
    }
}
