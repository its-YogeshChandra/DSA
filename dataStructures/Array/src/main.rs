mod alternate_elements;
use crate::alternate_elements::{alternate_elements, alternate_elements_recursive};

fn main() {
    let val = vec![10, 20, 30, 40, 50];
    alternate_elements(val.clone());

    alternate_elements_recursive(val);
}
