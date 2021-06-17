# Labyrinthian Possiblities

This is problem #17 found at https://binarysearch.com/problems/Labyrinthian-Possibilities

## Statement

There is a maze of cells - n rows by m columns. Starting from top-left find out how many ways there are to reach the bottom-right cell. You may only move right or down.

Mod the result to 10**9+7

The maze is represented as a matrix of numbers where 0 represents an empty space (you can move into) and 1 represents a wall (you cannot walk through). Assume the top-left and bottom-right will always be 0.

## Examples

**Input**
```
[
    [0, 0, 1],
    [0, 0, 1],
    [1, 0, 0],
]
```

**Output**
```
2
```
Starting from top-left, we can reach the bottom-right in the following ways:
```
right, down, down, right
down, right, down, right
```


**Input**
```
[
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
]
```

**Output**
```
6
```
Starting from top-left, we can reach the bottom-right in the following ways:
```
right, right, down, down
right, down, right, down
right, down, down, right
down, right, down, right
down, right, right, down
down, down, right, right
```
