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

fn main() {
    let n = 43;
    let value = find_even_or_odd(&n);
    if value == true {
        println!("the value is even ")
    } else {
        println!("the value is odd ")
    }
}

fn find_even_or_odd(n: &i32) -> bool {
    if n % 2 == 0 { true } else { false }
}
