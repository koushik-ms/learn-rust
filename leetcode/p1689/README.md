# Partitioning into minimum number of deci-binary numbers

This is problem #1689 found at https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

## Statement

Given a string 'n' representing a positive decimal integer, return the minimum number of positive deci-binary numbers needed so that they sum up to n.

A deci-binary number is a number made up of only 1s and 0s without any leading zeros - e.g., 101 and 110 are valid deci-binary numbers; 201 is not.

## Solution approach

since each number can have at most 1 at each digit (place) we need as many numbers as the largest value of any single digit in the original number. this can be a value between 0 and 9.

For 382, we need
111
111
110
 10
 10
 10
 10
 10

Notice how the first 2 numbers have 1 and the rest 0 in the right-most place. Similarly the first 3 have 1 and rest 0 in the left-most place. Of course, we omit any leading zeros.

However we need 8 numbers to be able to build up the 8 at the center. There is no way to reduce this since each digit of the deci-binary number can only have 1 at most.
