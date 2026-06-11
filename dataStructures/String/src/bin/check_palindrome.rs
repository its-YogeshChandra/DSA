// Palindrome String
// Last Updated : 8 Apr, 2026
// Given a string s, the task is to check if it is palindrome or not.
//
// Example:
//
// Input: s = "abba"
// Output: true
// Explanation: s is a palindrome
//
// Input: s = "abc"
// Output: false
// Explanation: s is not a palindromen

fn main() {
    let str = "abba";
    let result = check_palindrome(str);
    let opt_result = check_palindrome_optimal(str);
    println!("the result is : {}", result);
    println!("the opt result is : {}", opt_result)
}

//using two pointers
fn check_palindrome(main_str: &str) -> bool {
    let mut i = 0;
    let mut j = main_str.len() - 1;
    let str_arr: Vec<char> = main_str.chars().collect();

    while i < j {
        if str_arr[i] != str_arr[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn check_palindrome_optimal(main_str: &str) -> bool {
    let mut chars = main_str.chars();

    while let (Some(first), Some(last)) = (chars.next(), chars.next_back()) {
        if first != last {
            return false;
        }
    }

    true
}
