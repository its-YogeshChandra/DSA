// Find the Earliest Repeating Character
// Last Updated : 23 Jul, 2025
// Given a string S of length n, the task is to find the earliest repeated character in it. The earliest repeated character means, the character that occurs more than once and whose second occurrence has the smallest index.
//
// Example:
//
// Input: s = "geeksforgeeks"
// Output: e
// Explanation: e is the first element that repeats
//
// Input: s = "hello geeks"
// Output: l
// Explanation: l is the first element that repeats

fn main() {
    let s = "hello geeks";
    let result = first_repitition(s);
    println!("the result is : {}", result);
}

fn first_repitition(main_str: &str) -> char {
    let mut result: (usize, char, bool) = (0, '_', false);

    let mut i = 0;
    let mut j = 1;

    while i < (main_str.len() - 1) {
        if main_str.chars().nth(i).unwrap() == main_str.chars().nth(j).unwrap() {
            //check the difference
            let diff = j - i;

            //if diff i 1 then return the value immediately
            if diff == 1 {
                return main_str.chars().nth(i).unwrap();
            }

            //if diff is greater then
            if diff < result.0 {
                //update the result values
                result.0 = diff;
                result.1 = main_str.chars().nth(i).unwrap();
                result.2 = true;

                i += 1;
                j = i + 1;
            }
        }

        i += 1;
        j = i + 1;
    }

    if result.2 == false {
        println!("no element found");
        return ' ';
    } else {
        result.1
    }
}
