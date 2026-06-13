// Check if one string is subsequence of other
// Last Updated : 28 Mar, 2026
// Given two strings s1 and s2, find if the first string is a Subsequence of the second string, i.e. if s1 is a subsequence of s2.  A subsequence is a sequence that can be derived from another sequence by deleting some elements without changing the order of the remaining elements.
//
// Examples :
//
// Input: s1 = "AXY", s2 = "ADXCPY"
// Output: true
// Explanation: All characters of s1 are in s2 in the same order
//
// Input: s1 = "AXY", s2 = "YADXCP"
// Output: false
// Explanation: All characters are present, but order is not same.
//
// Input: s1 = "gksrek", s2 = "geeksforgeeks"
// Output: true

fn main() {
    let s1 = "AXY";
    let s2 = "YADXCP";
    let result = check_subseq(s1, s2);
    println!("the result is : {}", result);
    let opt_result = check_subseq_optimal(s1, s2);
    println!("the optimal result is : {}", opt_result)
}

//ook with flaw ( of not taking care of duplicate elements )
fn check_subseq(s1: &str, s2: &str) -> bool {
    let mut index = 0;
    for char in s1.chars() {
        //check if the value present or not
        if !s2.contains(char) {
            return false;
        }

        if index == 0 {
            index = s2.find(char).unwrap();
        } else {
            if s2.find(char).unwrap() < index {
                return false;
            } else {
                index = s2.find(char).unwrap();
            }
        }
    }

    true
}

//optimal approach for the problem
fn check_subseq_optimal(s1: &str, s2: &str) -> bool {
    let mut s2_iterator = s2.chars();

    s1.chars().all(|w| s2_iterator.any(|s2_char| s2_char == w))
}
