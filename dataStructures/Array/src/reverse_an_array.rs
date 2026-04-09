// Reverse an array arr[]. Reversing an array means rearranging the elements such that the first element becomes the last, the second element becomes second last and so on.
//
// Examples:
//
// Input: arr[] = [1, 4, 3, 2, 6, 5]
// Output:  [5, 6, 2, 3, 4, 1]
// Explanation: The first element 1 moves to last position, the second element 4 moves to second-last and so on.
//
// Input: arr[] = [4, 5, 1, 2]
// Output: [2, 1, 5, 4]
// Explanation: The first element 4 moves to last position, the second element 5 moves to second last and so on.

pub fn reverse_an_array(mut val: Vec<i32>) {
    //how to reverse an array
    let mut left_index = 0;
    let mut right_index = val.len() - 1;

    while left_index < right_index {
        [val[left_index], val[right_index]] = [val[right_index], val[left_index]];
        left_index += 1;
        right_index -= 1;
    }

    println!("the reverse array is : {:?}", val)
}
