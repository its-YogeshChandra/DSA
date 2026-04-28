// Check If Array Pair Sums Divisible by k
// Last Updated : 19 Nov, 2024
// Given an array of integers and a number k, write a function that returns true if the given array can be divided into pairs such that the sum of every pair is divisible by k.
//
// Examples:
//
// Input: arr[] = [9, 7, 5, 3], k = 6
// Output: True
// We can divide the array into (9, 3) and (7, 5). Sum of both of these pairs is a multiple of 6.
//
// Input: arr[] = [92, 75, 65, 48, 45, 35], k = 10
// Output: True
// We can divide the array into (92, 48), (75, 65) and (45, 35). The sum of all these pairs are multiples of 10.
//
// Input: arr[] = [91, 74, 66, 48], k = 10
// Output: False

use std::collections::HashMap;

fn main() {
    let arr = vec![92, 75, 65, 48, 45, 35];
    let n = 10;
    let result = divisible_by_k(arr, n);
    if result == true {
        println!("true")
    } else {
        println!("false")
    }
}

fn divisible_by_k(arr: Vec<i32>, n: i32) -> bool {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    for val in arr {
        let rem = val % n;
        hashmap.insert(rem, val);
    }

    for (val, _) in &hashmap {
        let rem = n - (val % n);
        if !hashmap.contains_key(&rem) {
            return false;
        }
    }

    true
}
