// LLongest Subarray With Sum Divisible By K
// Last Updated : 3 Feb, 2026
// Given an arr[] of integers and a positive integer k, find the longest subarray's length with the sum of the elements divisible by k.
//
// Examples:
//
// Input: arr[] = [2, 7, 6, 1, 4, 5], k = 3
// Output: 4
// Explanation: The subarray [7, 6, 1, 4] has sum = 18, which is divisible by 3.
//
// Input: arr[] = [-2, 2, -5, 12, -11, -1, 7], k = 3
// Output: 5
// Explanation: The subarray [2, -5, 12, -11, -1], has sum = -3, which is divisible by 3.
//
// Input: arr[] = [1, 2, -2], k = 5
// Output: 2
// Explanation: The subarray is [2, -2] with sum = 0, which is divisible by 5.ongest Subarray With Sum Divisible By K

use std::collections::HashMap;

fn main() {
    let arr = vec![2, 7, 6, 1, 4, 5];
    longest_subarray_divbyk(arr)
}

fn longest_subarray_divbyk(arr: Vec<i32>) {
    //find the longest sb arr div by k
    let mut sub_arr: Vec<i32> = vec![];
    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();

    //use two pointer in the single loop
    let i = 0;

    while i <= arr.len() - 1 {
        let j = i + 1;
        for index in j..arr.len() - 1 {}
    }
}
