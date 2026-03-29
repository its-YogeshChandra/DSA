// Given a positive integer n, find the sum of the first n natural numbers.
//
// Examples :
//
// Input: n = 3
// Output: 6
// Explanation: 1 + 2 + 3 = 6
//
// Input: n = 5
// Output: 15
// Explanation:  1 + 2 + 3 + 4 + 5 = 15
//

pub fn sum_of_n(n: i32) {
    //take the sum of n
    let mut result: i32 = 1;

    for ctr in 1..n {
        println!("ctr is : {}", ctr);
        result = result + (ctr + 1);
    }

    println!("the print is : {}", result)
}
