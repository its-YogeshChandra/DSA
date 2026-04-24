// Check for Disjoint Arrays or Sets
// Last Updated : 4 Feb, 2026
// Given two arrays a[] and b[], check if they are disjoint, i.e., there is no element common between both the arrays.
//
// Examples:
//
// Input: a[] = [12, 34, 11, 9, 3], b[] = [2, 1, 3, 5]
// Output: False
// Explanation: 3 is common in both the arrays.
//
// Input: a[] = [12, 34, 11, 9, 3], b[] = [7, 2, 1, 5]
// Output: True
// Explanation: There is no common element in both the arrays.

fn main() {
    let arr1 = vec![12, 34, 11, 9, 3];
    let arr2 = vec![2, 1, 3, 5];
    let result = check_disjoints(arr1, arr2);
    if result == true {
        println!("the result is true")
    } else {
        println!("the result is false")
    }
}

fn check_disjoints(arr1: Vec<i32>, arr2: Vec<i32>) -> bool {
    //check for the disjoints
    let modulo = arr1.len() * 2;
    let mut hash_arr: Vec<Option<i32>> = vec![None; modulo];
    let mut result = true;

    //hash the arr1
    //using linear probing
    for val in arr1 {
        let mut xmod = val % modulo as i32;

        while hash_arr[xmod as usize].is_some() {
            xmod = (xmod + 1) % modulo as i32
        }

        hash_arr[xmod as usize] = Some(val);
    }

    for val in arr2 {
        let mut xmod = val % modulo as i32;

        while hash_arr[xmod as usize].is_some() {
            if hash_arr[xmod as usize] == Some(val) {
                result = false;
                break;
            }
            xmod = (xmod + 1) % modulo as i32
        }
    }

    result
}
