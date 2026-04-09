// Given an array arr[], the task is to generate all the possible subarrays of the given array.
//
// Examples:
//
// Input: arr[] = [1, 2, 3]
// Output: [ [1], [1, 2], [2], [1, 2, 3], [2, 3], [3] ]
//
// Input: arr[] = [1, 2]
// Output: [ [1], [1, 2], [2] ]
//
// Try

pub fn generate_subarray(val: Vec<i32>) {
    let mut resultant_arr: Vec<Vec<i32>> = vec![];

    //add the first element of the array
    resultant_arr.push(vec![val[0]]);

    for (mut index, main_val) in val.clone().iter().enumerate() {
        //check if the resultant_arr contains things
        if !resultant_arr.contains(&vec![*main_val]) {
            resultant_arr.push(vec![*main_val])
        }

        if index == val.len() - 1 {
            break;
        }

        //create the dum array
        let mut dum_array: Vec<i32> = vec![*main_val];

        index += 1;

        for j in index..val.len() {
            dum_array.push(val[j]);
            resultant_arr.push(dum_array.clone())
        }
    }

    println!("the resultant_arr is : {:?}", resultant_arr)
}
