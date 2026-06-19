// First non-repeating character of given string
// Last Updated : 7 Apr, 2025
// Given a string s of lowercase English letters, the task is to find the first non-repeating character. If there is no such character, return '$'.
//
// Examples:
//
// Input: s = "geeksforgeeks"
// Output: 'f'
// Explanation: 'f' is the first character in the string which does not repeat.
//
// Input: s = "racecar"
// Output: 'e'
// Explanation: 'e' is the only character in the string which does not repeat.
//
// Input: "aabbccc"
// Output: '$'
// Explanation: All the characters in the given string are repeating.

use std::collections::HashMap;

fn main() {
    let s = "racecar";
    let result = check_norep(s);
    let lowercase_result = lowercase_norep_check(s);
    println!("the result is : {}", result);
    println!("the lowercase result is : {}", lowercase_result);
}

fn check_norep(main_str: &str) -> char {
    let str_map = main_str.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    for char in main_str.chars() {
        if let Some(&val) = str_map.get(&char) {
            if val == 1 {
                return char;
            }
        }
    }

    '$'
}

//if the scenario is only lowecase english letters then can solve it through stack
fn lowercase_norep_check(main_str: &str) -> char {
    //create array of ixed length
    let mut count = [0; 26];

    for byte_val in main_str.bytes() {
        count[(byte_val - b'a') as usize] += 1
    }

    for ch in main_str.chars() {
        if count[(ch as usize) - ('a' as usize)] == 1 {
            return ch;
        }
    }

    '$'
}
