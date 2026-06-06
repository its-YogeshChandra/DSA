// Check if given String is Pangram or not
// Last Updated : 17 Feb, 2026
// Given a string s, check if it is Pangram or not.
// A pangram is a sentence containing all letters of the English Alphabet.
//
// Examples:
//
// Input: s = "The quick brown fox jumps over the lazy dog"
// Output: true
// Explanation: The input string contains all characters from 'a' to 'z'.
//
// Input: s = "The quick brown fox jumps over the dog"
// Output: false
// Explanation: The input string does not contain all characters from 'a' to 'z', as 'l', 'z', 'y' are missing

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    let result = check_pangram(s);
    println!("the result is : {}", result);
    let opt_result = opt_check_panagram(s);
    println!("the opt result is : {}", opt_result)
}

//brute force approach
fn check_pangram(main_str: &str) -> bool {
    for val in 97..123 {
        let letter = val as u8 as char;
        if !main_str.to_ascii_lowercase().contains(letter) {
            return false;
        }
    }
    true
}

//optimal approach using bitwise operations

fn opt_check_panagram(main_str: &str) -> bool {
    let mut audit_int: i32 = 0;
    for char in main_str.chars() {
        if char.is_ascii_alphabetic() {
            let char_index = char.to_ascii_lowercase() as u8 - b'a';
            audit_int |= 1 << char_index;
        }
    }
    let target = (1 << 26) - 1;
    audit_int == target
}
