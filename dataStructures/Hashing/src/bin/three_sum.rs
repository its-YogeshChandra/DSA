// 3 Sum - Count all triplets with given sum
// Last Updated : 23 Jul, 2025
// Given an array arr[] and a target value, the task is to find the count of triplets present in the given array having sum equal to the given target.
//
// Examples:
//
// Input: arr[] = [0, -1, 2, -3, 1], target = -2
// Output: 2
// Explanation: Two triplets that add up to -2 are:
// arr[0] + arr[3] + arr[4] = 0 + (-3) + (1) = -2
// arr[1] + arr[2] + arr[3] = (-1) + 2 + (-3) = -2
//
// Input: arr[] = [1, -2, 1, 0, 5], target = 1
// Output: 0
// Explanation: There is no triplet whose sum is equal to 1.

use std::collections::HashSet;

fn main() {
    let arr = vec![0, -1, 2, -3, 1];
    let target = -2 as i32;
    count_triplets(arr, target);
}

fn count_triplets(arr: Vec<i32>, target: i32) {
    //its a two way function
    let mut cnt = 0;

    for i in 0..(arr.len() - 2) {
        let int_target = target - arr[i];
    }
}

fn get_triplet_count(arr: Vec<i32>, idx: usize, target: i32) {
    let mut freq: HashSet<i32> = HashSet::new();

    for i in idx..arr.len() {}
}
