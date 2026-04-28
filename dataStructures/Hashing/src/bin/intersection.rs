// Intersection of two Arrays
// Last Updated : 27 Jul, 2025
// Given two arrays a[] and b[], find their intersection — the unique elements that appear in both. Ignore duplicates, and the result can be in any order.
//
// Input: a[] = [1, 2, 1, 3, 1], b[] = [3, 1, 3, 4, 1]
// Output: [1, 3]
// Explanation: 1 and 3 are the only common elements and we need to print only one occurrence of common elements
//
// Input: a[] = [1, 1, 1], b[] = [1, 1, 1, 1, 1]
// Output: [1]
// Explanation: 1 is the only common element present in both the arrays.
//
// Input: a[] = [1, 2, 3], b[] = [4, 5, 6]
// Output: []
// Explanation: No common element in both the arrays.n
//
//

use std::collections::HashMap;

fn main() {
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 5, 6];
    let value = intersection(arr1, arr2);
    println!("the resultant arr is : {:?}", value)
}

fn intersection(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    //the elements that are appear in both the arrays
    let mut resultant_arr: Vec<i32> = vec![];
    let hashmap1 = arr1.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let hashmap2 = arr2.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    if hashmap1.len() >= hashmap2.len() {
        for (index, _) in hashmap1 {
            if let Some(_value) = hashmap2.get(index) {
                resultant_arr.push(*index)
            }
        }
    } else {
        for (index, _) in hashmap2 {
            if let Some(_value) = hashmap1.get(index) {
                resultant_arr.push(*index)
            }
        }
    }
    resultant_arr
}
