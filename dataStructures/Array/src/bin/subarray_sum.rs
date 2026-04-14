// Sum of all Subarrays
// Last Updated : 22 Jul, 2025
// Given an integer array arr[], compute the sum of all possible sub-arrays of the array. A sub-array is a contiguous part of the array.
//
// Examples:
//
// Input: arr[] = [1, 4, 5, 3, 2]
// Output: 116
// Explanation: Sum of all possible subarrays of the array [1, 4, 5, 3, 2] is 116.
//
// Input: arr[] = [1, 2, 3, 4]
// Output: 50
// Explanation: Sum of all possible subarrays of the array [1, 2, 3, 4] is 50.

pub fn sum_of_subarray(val: Vec<i32>) -> i32 {
    //the main sum
    let mut main_sum: i32 = 0;

    for (index, item) in val.iter().enumerate() {
        //the sum should be like
        main_sum += *item * (index as i32 + 1) * (val.len() as i32 - index as i32)
    }
    main_sum
}
pub fn main() {
    let val = vec![1, 4, 5, 3, 2];
    let result = sum_of_subarray(val);
    println!("the value is : {}", result)
}
