// Camel case of a given sentence
// Last Updated : 23 Nov, 2024
// Given a sentence having lowercase characters, the task is to convert it to Camel Case. In Camel Case, words are joined without spaces, the first word keeps its original case, and each subsequent word starts with an uppercase letter.
//
// Examples:
//
// Input: "i got intern at geeksforgeeks"
// Output: "iGotInternAtGeeksforgeeks"
//
// Input: "here comes the garden"
// Output: "hereComesTheGarden"

fn main() {
    let input = "i got intern at geeksforgeeks";
    camel_case_conversion(input);
}

fn camel_case_conversion(main_str: &str) {
    let mut value = String::new();
    for (index, val) in main_str.split(" ").enumerate() {
        if index > 0 {
            let mut chars = val.chars();
            if let Some(first_word) = chars.next() {
                for c in first_word.to_uppercase() {
                    value.push(c);
                }
                value.push_str(chars.as_str());
            }
        } else {
            value.push_str(val);
        }
    }
    println!("the value is : {:#?}", value);
}
