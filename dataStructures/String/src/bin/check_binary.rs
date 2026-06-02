// Given a string s, the task is to check if it is a binary string or not. A binary string is a string which only contains the characters '0' and '1'.
//
// Examples:
//
// Input: s = "01010101010"
// Output: true
//
// Input: s = "geeks101"
// Output: false

fn main() {
    let s = "geeks101";

    let result = check_binary(s);
    let res1 = check_bin1(s);
    let res2 = check_bin2(s);
    println! {"the result is : {}", result};
    println! {"the res1 is : {}", res1};
    println! {"the res2 is : {}", res2};
}

//method id ok
//issue: high memory usage: "chars are 4 bytes unicode in rust"
fn check_binary(main_str: &str) -> bool {
    //traverse the string
    let ite_str: Vec<char> = main_str.chars().collect();
    for (_, char) in ite_str.iter().enumerate() {
        if *char != '0' && *char != '1' {
            return false;
        }
    }
    true
}

//function without high memory usage
fn check_bin1(main_str: &str) -> bool {
    for val in main_str.chars() {
        if val != '0' && val != '1' {
            return false;
        }
    }
    true
}

//functional programming way
fn check_bin2(main_str: &str) -> bool {
    main_str.chars().all(|c| c == '1' || c == '2')
}
