// Find the Earliest Repeating Character
// Last Updated : 23 Jul, 2025
// Given a string S of length n, the task is to find the earliest repeated character in it. The earliest repeated character means, the character that occurs more than once and whose second occurrence has the smallest index.
//
// Example:
//
// Input: s = "geeksforgeeks"
// Output: e
// Explanation: e is the first element that repeats
//
// Input: s = "hello geeks"
// Output: l
// Explanation: l is the first element that repeats

use std::collections::HashSet;

fn main() {
    let s = "mercenary";
    let result = first_repitition(s);
    let opt_result = opt_check_first_rep(s);
    println!("the result is : {}", result);
    println!("the opt_result is : {}", opt_result);
}

//kind a optimal + brute force (may be leaving some edge cases)
fn first_repitition(main_str: &str) -> char {
    let mut result: (usize, char, bool) = (usize::MAX, '_', false);

    let chars: Vec<char> = main_str.chars().collect();
    let len = chars.len();

    // Brute-force nested loops to check every pair
    for i in 0..len {
        for j in (i + 1)..len {
            if chars[i] == chars[j] {
                // The problem wants the smallest SECOND occurrence index (which is j)
                if j < result.0 {
                    result.0 = j;
                    result.1 = chars[i];
                    result.2 = true;
                }
            }
        }
    }

    if !result.2 {
        println!("no element found");
        return ' ';
    } else {
        result.1
    }
}

//optimal approach to the function
fn opt_check_first_rep(main_str: &str) -> char {
    let mut seen: HashSet<char> = HashSet::new();

    for char in main_str.chars() {
        if !seen.insert(char) {
            return char;
        }
    }

    println!("no repitition found");
    ' '
}
