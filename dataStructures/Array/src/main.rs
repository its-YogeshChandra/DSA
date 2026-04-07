mod alternate_elements;
mod leader_in_array;

//use crate::alternate_elements::{alternate_elements, alternate_elements_recursive};
use crate::leader_in_array::find_the_leader;
fn main() {
    //   let val = vec![10, 20, 30, 40, 50];
    // alternate_elements(val.clone());

    // alternate_elements_recursive(val.clone());
    //

    let find_val = vec![16, 17, 4, 3, 5, 2];

    find_the_leader(find_val)
}
