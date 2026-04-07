// Given an array arr[] of size n, the task is to find all the Leaders in the array. An element is a Leader if it is greater than or equal to all the elements to its right side.
//
// Note: The rightmost element is always a leader.
//
// Examples:
//
// Input: arr[] = [16, 17, 4, 3, 5, 2]
// Output: [17 5 2]
// Explanation: 17 is greater than all the elements to its right i.e., [4, 3, 5, 2], therefore 17 is a leader. 5 is greater than all the elements to its right i.e., [2], therefore 5 is a leader. 2 has no element to its right, therefore 2 is a leader.
//
// Input: arr[] = [1, 2, 3, 4, 5, 2]
// Output: [5 2]
// Explanation: 5 is greater than all the elements to its right i.e., [2], therefore 5 is a leader. 2 has no element to its right, therefore 2 is a leader.

pub fn find_the_leader(val: Vec<i32>) {
    //find the leader in the array
    let mut resultant_arr: Vec<i32> = vec![];

    for (index, main_num) in val.clone().iter().enumerate() {
        let num_val = (index + 1) as usize;

        if index == val.len() - 1 {
            resultant_arr.push(*main_num);
            break;
        }

        for j in num_val..val.len() as usize {
            if *main_num < val[j] as i32 {
                //update the resultant err
                break;
            }

            if j == val.len() - 1 {
                resultant_arr.push(*main_num)
            }
        }
    }

    println!("the leader arr is : {:?}", resultant_arr)
}
