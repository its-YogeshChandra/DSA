// Primality Test and School Method Algorithms
// Last Updated : 28 Mar, 2026
// Given a positive integer, check if the number is prime or not. A prime is a natural number greater than 1 that has no positive divisors other than 1 and itself. Examples of the first few prime numbers are {2, 3, 5, ...}
// Examples :
//
// Input:  n = 11
// Output: true
//
// Input:  n = 15
// Output: false
//
// Input:  n = 1
// Output: false
//

pub fn check_primality(n: i32) -> String {
    //divide the number from 2 to n-1

    for number in 2..n - 1 {
        if n % number == 0 {
            return "not a prime".to_string();
        }
    }
    "its a prime".to_string()
}
