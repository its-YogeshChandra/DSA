mod alternate_elements;
mod generate_subarray;
mod leader_in_array;
mod remove_duplicates;

//use crate::alternate_elements::{alternate_elements, alternate_elements_recursive};
use crate::{leader_in_array::find_the_leader, remove_duplicates::remove_duplicates_second};
use generate_subarray::generate_subarray;
use remove_duplicates::remove_duplicates;

fn main() {
    //   let val = vec![10, 20, 30, 40, 50];
    // alternate_elements(val.clone());

    // alternate_elements_recursive(val.clone());
    //

    //let find_val = vec![16, 17, 4, 3, 5, 2];
    //find_the_leader(find_val);

    // let sorted_arr = vec![2, 2, 2, 2, 2];
    // remove_duplicates(sorted_arr.clone());
    // remove_duplicates_second(sorted_arr.clone());

    //generate_subarray
    let subarray_val = vec![1, 2, 3];
    generate_subarray(subarray_val);
}
