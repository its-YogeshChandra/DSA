// Count of substrings that start and end with 1 in given Binary String
// Last Updated : 17 Feb, 2026
// Given a binary string s, the task is to count all substrings that start and end with the character '1'. A valid substring must have both its first and last characters as '1', and can include one or more number of characters in between.
//
// Examples:
//
// Input: s = "00100101"
// Output: 3
// Explanation: Valid substrings are "1001", "100101", and "101", all starting and ending with '1'.
//
// Input: s = "1001"
// Output: 1
// Explanation: Only one valid substring: "1001", which starts and ends with '1'.
//
// Input: s = "111"
// Output: 3
// Explanation: Valid substrings are "11" (first and second), "11" (second and third), and "111" (entire string).
//

use std::collections::HashMap;

fn main() {
    let s = "111";
    let result = check_substring(s);
    println!("the result is : {}", result);
    let op_result = optimal_check_substring(s);
    println!("the op_result is : {}", op_result);
}

//brute force
fn check_substring(main_str: &str) -> i32 {
    let mut ctr = 0;
    for (idx, char) in main_str.chars().enumerate() {
        if char == '1' {
            for i in (idx + 1)..main_str.len() {
                if main_str.chars().nth(i).unwrap() == '1' {
                    ctr += 1;
                }
            }
        }
    }
    ctr
}

//optimal approach
//using combinatorial formula
fn optimal_check_substring(main_str: &str) -> i32 {
    let mut char_hashmap = HashMap::new();
    for char in main_str.chars() {
        *char_hashmap.entry(char).or_insert(0) += 1;
    }
    //check the thing
    match char_hashmap.get(&'1') {
        Some(value) => {
            let substring = value * (value - 1) / 2;
            substring
        }
        None => {
            println!("no instance of target");
            0
        }
    }
}
