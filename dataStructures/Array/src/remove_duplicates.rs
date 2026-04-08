// Remove duplicates from Sorted Array
// Last Updated : 19 Nov, 2024
// Given a sorted array arr[] of size n, the goal is to rearrange the array so that all distinct elements appear at the beginning in sorted order. Additionally, return the length of this distinct sorted subarray.
//
// Note: The elements after the distinct ones can be in any order and hold any value, as they don't affect the result.
//
// Examples:
//
// Input: arr[] = [2, 2, 2, 2, 2]
// Output: [2]
// Explanation: All the elements are 2, So only keep one instance of 2.
//
// Input: arr[] = [1, 2, 2, 3, 4, 4, 4, 5, 5]
// Output: [1, 2, 3, 4, 5]
//
// Input: arr[] = [1, 2, 3]
// Output: [1, 2, 3]
// Explanation : No change as all elements are distinct.
//

pub fn remove_duplicates(val: Vec<i32>) {
    //iterate over the array and remove the duplicate values
    let mut resultant_arr: Vec<i32> = vec![];

    for (mut index, main_val) in val.clone().iter().enumerate() {
        //second loop to compare the value
        index += 1;
        for j in index..val.len() {
            if *main_val == val[j] {
                if resultant_arr.contains(main_val) == false {
                    resultant_arr.push(*main_val)
                }
            }
        }
    }
    println!("the resultant arr is : {:?}", resultant_arr)
}
