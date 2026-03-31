// Sum of Digits of a Number
// Last Updated : 14 Jul, 2025
// Given a number n, find the sum of its digits.
//
// Examples :
//
// Input: n = 687
// Output: 21
// Explanation: The sum of its digits are: 6 + 8 + 7 = 21
//
// Input: n = 12
// Output: 3
// Explanation: The sum of its digits are: 1 + 2 = 3
//
//

pub fn sum_of_digits(n: i32) {
    //return the sum of digits
    let num_string = n.to_string();
    let mut result_val = 0;

    for num in num_string.chars() {
        //convert the string num to int num

        let val: i32 = num.to_string().parse().unwrap();
        result_val = result_val + val
    }

    println!("the result_val is : {}", result_val);
}
