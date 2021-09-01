# Odd Palindrome

This is problem #472 found at https://binarysearch.com/problems/Increasing-Digits

## Statement

Given a number `n` find the number of positive integers with n digits that have strictly increasing digits.

### Examples

**Input**
```
n = 2
```
**Output**
```
36
```
**Input**
```
n = '3'
```
**Output**
```
84
```

## Solution

### Approach 1

For n=1, there are 9 numbers, {1, 2, 3, 4, 5, 6, 7, 8, 9}. Thus 1 number for each ending digit.
For n=2, we can pick each number in the set for n=1 and prefix all numbers smaller than first digit. Of course we have to discard 1 and start with all other numbers this way, we get:
    12                   => 1
    13, 23               => 2
    14, 24, 34           => 3
    ...                  
    19, 29, 39, ..., 89  => 8
    So there are, in total, 1 + 2 + 3 + ... + 8 = 36 combinations
    This is the partial sum of the sequence for n = 1 (omitting the last element)
For n=3, we can similarly see that we have to ignore numbers starting with 1. we can prefix 1 for all numbers starting with 2, 3, ... 8. We can prefix 2 for all numbers starting with 3, 4, ... , 8 and so on. We can prefix 6 to 78, 79 and 89. We can prefix 7 only to 89. So the number of such 3-digit number is equal to 1 + 3 + 6 + 10 + 15 + 21 + 28. This is again a partial sum of the sequence for n=2 (omitting the last element).

We can use this to make a dp solution where we can calculate from 1 upto n digits iteratively.

### Approach 2
By picking `n` (distinct) digits we can make an n-digit number. Each of these digits should be a number between 1 and 9 (inclusive).
Since these digits are distinct (no repetition) there is exactly one way to arrange them - hence there is only 1 n-digit number with strictly increasing digits given any `n` numbers between 1 and 9.

Thus, the number of n-digit numbers with strictly increasing digits is 9Cn - the number of ways of choosing n numbers from a set of 9 numbers i.e., {1, 2, 3, 4, 5, 6, 7, 8, 9}.
