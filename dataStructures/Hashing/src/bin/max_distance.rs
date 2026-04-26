// Max Distance Between Two Occurrences
// Last Updated : 23 Jul, 2025
// Given an array arr[], the task is to find the maximum distance between two occurrences of any element. If no element occurs twice, return 0.
//
// Examples:
//
// Input: arr = [1, 1, 2, 2, 2, 1]
// Output: 5
// Explanation: distance for 1 is: 5-0 = 5, distance for 2 is: 4-2 = 2, So max distance is 5.
//
// Input : arr[] = [3, 2, 1, 2, 1, 4, 5, 8, 6, 7, 4, 2]
// Output: 10
// Explanation : Max distance for 2 is 11-1 = 10, max distance for 1 is 4-2 = 2 and max distance for 4 is 10-5 = 5
//
// Input: arr[] = [1, 2, 3, 6, 5, 4]
// Output: 0
// Explanation: No element has two occurrence, so maximum distance = 0.

use std::collections::HashMap;

fn main() {
    let arr: Vec<i32> = vec![3, 2, 1, 2, 1, 4, 5, 8, 6, 7, 4, 2];
    let value = max_distance(arr);
    println!("the result is : {}", value)
}

fn max_distance(arr1: Vec<i32>) -> i32 {
    //loop the arr1
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;

    //iterate over the arr
    for (index, val) in arr1.iter().enumerate() {
        if let Some(main_val) = hashmap.get(&val) {
            let new_val = index as i32 - main_val;
            if new_val > *main_val && new_val > result {
                result = new_val
            }
        } else {
            hashmap.insert(*val, index as i32);
        }
    }

    return result;
}
