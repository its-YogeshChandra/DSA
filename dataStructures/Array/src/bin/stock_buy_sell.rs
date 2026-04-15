// Stock Buy and Sell - Multiple Transaction Allowed
// Last Updated : 9 Feb, 2026
// Given an array prices[] representing stock prices, find the maximum total profit that can be earned by buying and selling the stock any number of times.
//
// Note: We can only sell a stock which we have bought earlier and we cannot hold multiple stocks on any day.
//
// Examples:
//
// Input: prices[] = [100, 180, 260, 310, 40, 535, 695]
// Output: 865
// Explanation: Buy the stock on day 0 and sell it on day 3 = 310 - 100 = 210 and Buy the stock on day 4 and sell it on day 6 = 695 - 40 = 655 so the Maximum Profit  is = 210 + 655 = 865.
//
// total_profit
//
// Input: prices[] = [4, 2]
// Output: 0
// Explanation: Stock prices keep decreasing, there is no chance to sell at a higher price after buying, so no profit can be made.

fn max_profit(val: Vec<i32>) -> i32 {
    let mut main_profit = 0;

    //only any day is :
    for index in 1..val.len() {
        if val[index] > val[index - 1] {
            main_profit = main_profit + (val[index] - val[index - 1])
        }
    }
    main_profit
}

fn main() {
    let val: Vec<i32> = vec![100, 180, 260, 310, 40, 535, 695];

    let result = max_profit(val);
    println!("the result is : {}", result)
}
