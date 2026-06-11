// Check if a string is substring of another
// Last Updated : 17 Jan, 2026
// Given two strings txt and pat, the task is to find if pat is a substring of txt. If yes, return the index of the first occurrence, else return -1.
//
// Examples :
//
// Input: txt = "geeksforgeeks", pat = "eks"
// Output: 2
// Explanation: String "eks" is present at index 2 and 10, so 2 is the smallest index.
//
// Input: txt = "geeksforgeeks", pat = "xyz"
// Output: -1.
// Explanation: There is no occurrence of "xyz" in "geeksforgeeks"

use std::collections::HashSet;

fn main() {
    //check substring
    let txt = "geeksforgeeks";
    let pat = "eks";
    let result = check_substring(txt, pat);
    let tr_result = check_substring(txt, pat);

    println!("the result is : {}", result);
    println!("the tr_result is : {}", tr_result)
}

//optimal way
fn check_substring(main_str: &str, sub_str: &str) -> bool {
    let str_hashset: HashSet<char> = main_str.chars().collect();

    for char in sub_str.chars() {
        if !str_hashset.contains(&char) {
            return false;
        }
    }
    true
}

//if looking for true substring
fn true_substring(main_str: &str, sub_str: &str) -> bool {
    main_str.contains(sub_str)
}
