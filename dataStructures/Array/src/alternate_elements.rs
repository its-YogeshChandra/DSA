// Alternate elements of an array
// Last Updated : 4 Dec, 2024
// Given an array arr[], the task is to print every alternate element of the array starting from the first element.
//
// Examples:
//
// Input: arr[] = [10, 20, 30, 40, 50]
// Output: 10 30 50
// Explanation: Print the first element (10), skip the second element (20), print the third element (30), skip the fourth element(40) and print the fifth element(50).
//
// Input: arr[] = [-5, 1, 4, 2, 12]
// Output: -5 4 12
//

pub fn alternate_elements(val: Vec<i32>) {
    //print the alternate elements
    let mut ctr = 1;
    for num in val {
        if ctr % 2 != 0 {
            println!("the value is : {}", num);
        }
        ctr += 1
    }
}

pub fn alternate_elements_recursive(val: Vec<i32>) {
    let mut ctr = 0;
    main_recursion(val, &mut ctr);
}

pub fn main_recursion(val: Vec<i32>, ctr: &mut i32) {
    //base condition
    if *ctr >= val.len() as i32 {
        return;
    }

    let main_val = val[*ctr as usize];

    if *ctr % 2 == 0 {
        println! {"the main_val is : {}", main_val }
    };

    *ctr += 1;
    main_recursion(val, ctr);
}
