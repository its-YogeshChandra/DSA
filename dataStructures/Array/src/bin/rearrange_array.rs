// Rearrange array such that even positioned are greater than odd
// Last Updated : 27 Jan, 2026
// Given an array arr[], sort the array according to the following relations:
//
// arr[i] >= arr[i - 1], if i is even, ∀ 1 <= i < n
// arr[i] <= arr[i - 1], if i is odd, ∀ 1 <= i < n
// Find the resultant array.[consider 1-based indexing]
//
// Examples:
//
// Input: arr[] = [1, 2, 2, 1]
// Output: [1 2 1 2]
//  Explanation:
// For i = 2, arr[i] >= arr[i-1]. So, 2 >= 1.
// For i = 3, arr[i] <= arr[i-1]. So, 1 <= 2.
// For i = 4, arr[i] >= arr[i-1]. So, 2 >= 1.
//
// Input: arr[] = [1, 3, 2]
// Output: [1 3 2]
// Explanation:
// For i = 2, arr[i] >= arr[i-1]. So, 3 >= 1.
// For i = 3, arr[i] <= arr[i-1]. So, 2 <= 3.
//

fn rearrange_array(mut val: Vec<i32>) -> Vec<i32> {
    //use 1 based indexing
    //the values at even postions should be greater then that on the odd poistions
    let mut index = 1;

    loop {
        if index % 2 != 0 {
            if val[index] < val[index - 1] {
                [val[index], val[index - 1]] = [val[index - 1], val[index]];
            }
        } else {
            if val[index] > val[index - 1] {
                [val[index], val[index - 1]] = [val[index - 1], val[index]]
            }
        }

        if index >= val.len() - 1 {
            break;
        }

        index += 1;
    }

    println!("the val is : {:#?}", val);
    val
}

fn main() {
    let val = vec![1, 2, 2, 1];
    let val_two = vec![1, 3, 2];

    rearrange_array(val_two);
}
