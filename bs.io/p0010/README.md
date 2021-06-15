# A Flight of Stairs

This is problem #10 found at https://binarysearch.com/problems/A-Flight-of-Stairs

## Statement

There's a staircase with `n` steps. You can climb either 1 or 2 steps at a time. In how many different ways can you climb the staircase ? 

The order of single and double steps matters. So, e.g., a double-step followed by a single step is different from a double step preceded by a single step.

Mod the result to 10**9+7

## Examples

**Input**
```
n = 4
```

**Output**
```
5
```
With 4 steps in the staircase, it can be climed in the following ways:
```
1,1,1,1 => visit step 1, 2, 3, 4
1,2,1 => visit step 1, 3, 4
1,1,2 => visit step 1, 2, 4
2,1,1 => visit step 2, 3, 4
2,2 => visit step 2, 4
```


**Input**
```
n = 5
```

**Output**
```
8
```
With 4 steps in the staircase, it can be climed in the following ways:
```
1,1,1,1,1 => visit step 1, 2, 3, 4, 5
1,1,1,2 => visit step 1, 2, 3, 5
1,1,2,1 => visit step 1, 2, 4, 5
1,2,1,1 => visit step 1, 3, 4, 5
2,1,1,1 => visit step 2, 3, 4, 5
2,2,1 => visit step 2, 4, 5
2,1,2 => visit step 2, 3, 5
1,2,2 => visit step 1, 3, 5
```

