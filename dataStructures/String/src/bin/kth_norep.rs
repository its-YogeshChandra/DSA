// K'th Non-repeating Character
// Last Updated : 15 Jul, 2024
// Given a string str of length n (1 <= n <= 106) and a number k, the task is to find the kth non-repeating character in the string.
//
// Examples:
//
// Input : str = geeksforgeeks, k = 3
// Output : r
// Explanation: First non-repeating character is f, second is o and third is r.
//
// Input : str = geeksforgeeks, k = 2
// Output : o
//
// Input : str = geeksforgeeks, k = 4
// Output : Less than k non-repeating characters in input.
use std::collections::HashMap;

fn main() {
    let str_val = "geeksforgeeks";
    let k = 4;

    if let Some(ch) = check_kthnorep(str_val, k) {
        println!("the char is : {}", ch);
    } else {
        println!("less than k non-repeating characters in input");
    }
}

fn check_kthnorep(main_str: &str, k: u16) -> Option<char> {
    let str_map = main_str.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut seen_non_repeating = 0;

    for char in main_str.chars() {
        if let Some(&val) = str_map.get(&char) {
            if val == 1 {
                seen_non_repeating += 1;

                // Once our counter matches k, return immediately
                if seen_non_repeating == k {
                    return Some(char);
                }
            }
        }
    }

    None
}
