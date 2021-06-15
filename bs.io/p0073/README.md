# Sudoku Solver

This is problem #73 found at https://binarysearch.com/problems/Sudoku-Solver

## Statement

Implement a [Sudoku](https://en.wikipedia.org/wiki/Sudoku) solver that takes an incomplete board and solves it. In the given board, incomplete squares will be marked with a 0.

Assume the board has only one solution.

## Examples

**Input**
```
board = [
    [0, 2, 0, 5, 0, 1, 0, 9, 0],
    [8, 0, 0, 2, 0, 3, 0, 0, 6],
    [0, 3, 0, 0, 6, 0, 0, 7, 0],
    [0, 0, 1, 0, 0, 0, 6, 0, 0],
    [5, 4, 0, 0, 0, 0, 0, 1, 9],
    [0, 0, 2, 0, 0, 0, 7, 0, 0],
    [0, 9, 0, 0, 3, 0, 0, 8, 0],
    [2, 0, 0, 8, 0, 4, 0, 0, 7],
    [0, 1, 0, 9, 0, 7, 0, 6, 0]
]
```

**Output**
```
[
    [4, 2, 6, 5, 7, 1, 3, 9, 8],
    [8, 5, 7, 2, 9, 3, 1, 4, 6],
    [1, 3, 9, 4, 6, 8, 2, 7, 5],
    [9, 7, 1, 3, 8, 5, 6, 2, 4],
    [5, 4, 3, 7, 2, 6, 8, 1, 9],
    [6, 8, 2, 1, 4, 9, 7, 5, 3],
    [7, 9, 4, 6, 3, 2, 5, 8, 1],
    [2, 6, 5, 8, 1, 4, 9, 3, 7],
    [3, 1, 8, 9, 5, 7, 4, 6, 2]
]
```

## Notes

Notes about implementation

### Run tests with output (from print! etc)

Use the `--nocapture` flag as outlined [here](https://medium.com/@ericdreichert/how-to-print-during-rust-tests-619bdc7ccebc)

```bash
$ cargo test -- --nocapture
```
