// Check if Strings Are Rotations of Each Other
// Last Updated : 3 Oct, 2025
// Given two strings s1 and s2 of equal length, determine whether s2 is a rotation of s1.
// A string is said to be a rotation of another if it can be obtained by shifting some leading characters of the original string to its end without changing the order of characters.
//
// Examples:
//
// Input: s1 = "abcd", s2 = "cdab"
// Output: true
// Explanation: After 2 right rotations, s1 will become equal to s2.
//
// Input: s1 = "aab", s2 = "aba"
// Output: true
// Explanation: After 1 left rotation, s1 will become equal to s2.
//
// Input: s1 = "abcd", s2 = "acbd"
// Output: false
// Explanation: Strings are not rotations of each other.

fn main() {
    let s1 = "abcd".to_string();
    let s2 = "cdab";
    let result = check_ifrotation(s1.clone(), s2);
    let opt_result = optimal_check_rotation(s1.as_str(), s2);
    println!("the result is: {}", result);
    println!("the opt_result is: {}", opt_result);
}

fn check_ifrotation(mut s1: String, s2: &str) -> bool {
    //condition 1
    if s1.len() != s2.len() {
        return false;
    }

    //condition 2
    if s1 == s2.to_string() {
        return false;
    }

    //condition 1 and 2 can be merged into a single condition

    let mut _rotation_count = 0 as usize;
    let max_rotation = s1.chars().count();
    //     if rotation_count == max_rotation {
    //         println!("called stopper");
    //         break;
    //     } else {
    //         println!("called incrementer");
    //         rotation_count += 1
    //     }
    //     let mut chars_it = s1.chars();
    //     if let Some(value) = chars_it.next() {
    //         let mut new_str: String = chars_it.collect();
    //
    //         new_str.push(value);
    //
    //         s1 = new_str;
    //     }
    //
    //     println!("the value of s1 is : {}", s1);
    //     if s1 == s2.to_string() {
    //         println!("checker called");
    //         return true;
    //     }
    // }

    //by looping max rotation values
    for _ in 0..max_rotation {
        let mut chars_it = s1.chars();

        if let Some(value) = chars_it.next() {
            let mut new_str: String = chars_it.collect();

            new_str.push(value);

            s1 = new_str;
        }
        println!("s1 is : {}", s1);
        if s1 == s2.to_string() {
            return true;
        }
    }

    false
}

//optimal and interesting
fn optimal_check_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() || s1 == s2 {
        return false;
    }

    let doubled = format! {"{}{}", s1,s1};

    if doubled.contains(s2) {
        return true;
    }

    false
}
