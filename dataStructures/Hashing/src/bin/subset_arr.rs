// Check if an array is subset of another array
// Last Updated : 4 Feb, 2026
// Given two arrays a[] and b[] of size m and n respectively, the task is to determine whether b[] is a subset of a[]. Both arrays are not sorted, and elements are distinct.
//
// Examples:
//
// Input: a[] = [11, 1, 13, 21, 3, 7], b[] = [11, 3, 7, 1]
// Output: true
//
// Input: a[]= [1, 2, 3, 4, 5, 6], b = [1, 2, 4]
// Output: true
//
// Input: a[] = [10, 5, 2, 23, 19], b = [19, 5, 3]
// Output: false
//

fn main() {
    let arr1: Vec<i32> = vec![11, 1, 13, 21, 3, 7];
    let arr2: Vec<i32> = vec![11, 3, 7, 1];
    let result = subset_of_array(arr1, arr2);

    if result == true {
        println!("is subset of the array")
    } else {
        println!("isn't subset of the array")
    }
}

fn subset_of_array(arr1: Vec<i32>, arr2: Vec<i32>) -> bool {
    //check if the  val of the b exist in the val of the another
    let modulo = arr1.len() * 2;
    let mut hash_arr: Vec<Option<i32>> = vec![None; modulo];

    //loop to has the arr1
    for val in arr1 {
        //use the liner probing
        let mut xmod = val % modulo as i32;
        println!("the value of the hasharr is  : {:?}", hash_arr);

        while hash_arr[xmod as usize].is_some() {
            xmod = (xmod + 1) % modulo as i32
        }

        hash_arr[xmod as usize] = Some(val);
    }

    //loop through second array and check if has_arr didn't contains the val
    for val in arr2 {
        let mut xmod = val % modulo as i32;
        let mut result = false;

        while hash_arr[xmod as usize].is_some() {
            //check if there is the value
            if hash_arr[xmod as usize] == Some(val) {
                result = true;
                break;
            }

            xmod = (xmod + 1) % modulo as i32
        }

        if result == false {
            return false;
        }
    }

    true
}
