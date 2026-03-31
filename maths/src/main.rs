//starting of the dsa series

// Given a number n, check whether it is even or odd. Return true for even and false for odd.
//
// Examples:
//
// Input: n = 15
// Output: false
// Explanation: 15 % 2 = 1, so 15 is odd .
//
// Input: n = 44
// Output: true
// Explanation: 44 % 2 = 0, so 44 is even.
mod multiplication;
mod reverse_a_digits;
mod sum_of_digits;
mod sum_of_n;

use multiplication::print_table;
use reverse_a_digits::reverse_a_digit;
use sum_of_digits::sum_of_digits;
use sum_of_n::sum_of_n;

fn main() {
    let n = 43;
    let value = find_even_or_odd(&n);
    if value == true {
        println!("the value is even ")
    } else {
        println!("the value is odd ")
    }

    //call the multiplication table
    print_table(6);

    //call the sum of n
    sum_of_n(5);

    //call the sum of digits
    sum_of_digits(345);

    //reverse_a_digit
    reverse_a_digit(456);
}

fn find_even_or_odd(n: &i32) -> bool {
    if n % 2 == 0 { true } else { false }
}
