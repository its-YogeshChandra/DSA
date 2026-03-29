// Given a number n, we need to print its table.
//
// Examples :
//
// Input:  5
// Output:
// 5 * 1 = 5
// 5 * 2 = 10
// 5 * 3 = 15
// 5 * 4 = 20
// 5 * 5 = 25
// 5 * 6 = 30
// 5 * 7 = 35
// 5 * 8 = 40
// 5 * 9 = 45
// 5 * 10 = 50
//
// Input:  2
// Output:
// 2 * 1 = 2
// 2 * 2 = 4
// 2 * 3 = 6
// 2 * 4 = 8
// 2 * 5 = 10
// 2 * 6 = 12
// 2 * 7 = 14
// 2 * 8 = 16
// 2 * 9 = 18
// 2 * 10 = 20
//
//

//crate function for this
pub fn print_table(n: i32) {
    //loop and multiply again and again
    for number in 1..10 {
        let value = number * n;
        println!("{} times {} is {}", number, n, value);
    }
}
