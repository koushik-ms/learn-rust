# Friend Groups

This is problem #150 found at https://binarysearch.com/problems/Friend-Groups

## Statement
Given an undirected graph `friends` as an [adjacency list](https://en.wikipedia.org/wiki/Adjacency_list), where `friends[i]` is a list of people `i` is friends with. Friendships are mutual and two people are in a friend group when there is some path of mutual friends connecting them.

Return the number of friend groups in `friends`.

## Examples

**Input**
```
friends = [
    [1, 2],
    [],
    [0],
    [4],
    []
]
```

**Output**
```
2
```

## Notes

Notes about implementation

### Analysis 

Given a graph of friends where nodes represent people and edges represent friendships, a friend group is essentially a connected component.
The solution is using the Union-find algorithm to find connected components.


### Run tests with output (from print! etc)

Use the `--nocapture` flag as outlined [here](https://medium.com/@ericdreichert/how-to-print-during-rust-tests-619bdc7ccebc)

```bash
$ cargo test -- --nocapture
```
