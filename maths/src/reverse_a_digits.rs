// Write a program to reverse digits of a number
// Last Updated : 21 May, 2025
// Given an Integer n, find the reverse of its digits.
//
// Examples:
//
// Input: n = 122
// Output: 221
// Explanation: By reversing the digits of number, number will change into 221.
//
// Input: n = 200
// Output: 2
// Explanation: By reversing the digits of number, number will change into 2.
//
// Input: n = 12345
// Output: 54321
// Explanation: By reversing the digits of number, number will change into 54321.
//

pub fn reverse_a_digit(n: i32) {
    //reverse a digit
    let mut char_array: Vec<char> = vec![];

    for nums in n.to_string().chars() {
        //get the num
        char_array.push(nums)
    }

    char_array.reverse();
    let val: String = char_array.iter().collect();

    println!("the reverse number is : {}", val.parse::<i32>().unwrap())
}
