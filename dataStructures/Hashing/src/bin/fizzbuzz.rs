// Fizz Buzz
// Last Updated : 23 Jul, 2025
// Given an integer n, for every positive integer i <= n, the task is to print,
//
// "FizzBuzz" if i is divisible by 3 and 5,
// "Fizz" if i is divisible by 3,
// "Buzz" if i is divisible by 5
// "i" as a string, if none of the conditions are true.
// Examples:
//
// Input: n = 3
// Output: ["1", "2", "Fizz"]
//
// Input: n = 10
// Output: ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz"]
//
// Input: n = 20
// Output: ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz"]

use std::collections::HashMap;

fn main() {
    let n = 20;
    fizz_buzz(n);
}

fn fizz_buzz(mut n: isize) {
    //for
    //divisible by 3
    //divisible by 5
    //divisible by 3 and 5 both
    let mut hash_arr: Vec<String> = vec![];
    n += 1;
    for i in 1..n {
        if i % 3 == 0 {
            //check for reverse
            if i % 5 == 0 {
                hash_arr.push("FizzBuzz".to_string());
            }
            hash_arr.push("Fizz".to_string())
        } else if i % 5 == 0 {
            //check for reverse
            if i % 3 == 0 {
                hash_arr.push("FizzBuzz".to_string())
            }

            hash_arr.push("Buzz".to_string())
        } else {
            hash_arr.push(i.to_string())
        }
    }
    println!("{:?}", hash_arr)
}
