// Check if two arrays are equal or not
// Last Updated : 5 Feb, 2026
// Given two arrays, a[] and b[] of equal length. The task is to determine if the given arrays are equal or not. Two arrays are considered equal if:
//
// Both arrays contain the same set of elements.
// The arrangements (or permutations) of elements may be different.
// If there are repeated elements, the counts of each element must be the same in both arrays.
// Examples:
//
// Input: a[] = [1, 2, 5, 4, 0], b[] = [2, 4, 5, 0, 1]
// Output: true
//
// Input: a[] = [1, 2, 5, 4, 0, 2, 1], b[] = [2, 4, 5, 0, 1, 1, 2]
// Output: true
//
//  Input: a[] = [1, 7, 1], b[] = [7, 7, 1]
// Output: false
use std::collections::HashMap;

fn main() {
    let arr1 = vec![1, 2, 5, 4, 0, 2, 1];
    let arr2 = vec![2, 4, 5, 0, 1, 1, 2];
    let result = check_equals(arr1, arr2);
    if result == true {
        println!("the arr2 is equal to arr1")
    } else {
        println!("the arr2 is not equal to arr1")
    }
}

fn check_equals(arr1: Vec<i32>, arr2: Vec<i32>) -> bool {
    let mut hash_body = arr1.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    //use the linear probing on arr 2
    for val in arr2 {
        if let Some(count) = hash_body.get_mut(&val) {
            if *count == 0 {
                return false;
            }
            *count -= 1;
        } else {
            return false;
        }
    }

    true
}
