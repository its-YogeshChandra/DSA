// Duplicate within K Distance in an Array
// Last Updated : 23 Jul, 2025
// Given an integer array arr[] and an integer k, determine whether there exist two indices i and j such that arr[i] == arr[j] and |i - j| ≤ k. If such a pair exists, return 'Yes', otherwise return 'No'.
//
// Examples:
//
// Input: k = 3, arr[] = [1, 2, 3, 4, 1, 2, 3, 4]
// Output: No
// Explanation: Each element in the given array arr[] appears twice and the distance between every element and its duplicate is 4.
//
// Input: k = 3, arr[] = [1, 2, 3, 1, 4, 5]
// Output: Yes
// Explanation: 1 is present at index 0 and 3.
//
// Input: k = 3, arr[] = [1, 2, 3, 4, 5]
// Output: No
// Explanation: There is no duplicate element in arr[].

pub fn duplicate_in_k_distance(val: Vec<i32>, k: usize) -> bool {
    //iterate over the array
    let i = 0;

    main_recursion(val, k, i)
}

pub fn main_recursion(val: Vec<i32>, k: usize, mut i: usize) -> bool {
    //write the base condition
    if i == val.len() - 1 {
        return false;
    }

    let mut j = val.len() - 1;

    let fin: bool = true;

    while i < j {
        if val[i] == val[j] {
            if (j - i) <= k {
                return fin;
            } else {
                j -= 1
            }
        } else {
            j -= 1
        }
    }
    i += 1;

    //the main recursion
    main_recursion(val, k, i)
}
