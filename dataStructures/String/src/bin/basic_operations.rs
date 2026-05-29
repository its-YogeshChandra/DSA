//basic operations
fn main() {
    let val = "GeeksforGeeks";
    let result = find_len(val);
    println!("the result is : {}", result);

    //check for same
    println!("check for same");
    let i = "fight";
    let j = "fighters";
    let same_res = check_for_same(i, j);
    println!("same_res is : {}", same_res);

    //check for presence
    println!("-----");
    println!("check for presence");
    println!("-----");
    let ch = "k";
    let str_val = "GeeksforGeeks";
    let pre_res = check_presence(ch, str_val);
    println!("presence index is : {}", pre_res);

    //insert character
    println!("-----");
    println!("check for presence");
    println!("-----");
    let the_string = "Geeks";
    let ch = "askin";
    let pos = 3 as usize;
}

//find the length
fn find_len(val: &str) -> usize {
    //return the length
    val.len()
}

//check for same
fn check_for_same(i: &str, j: &str) -> bool {
    i == j
}

//check presence
fn check_presence(ch: &str, str_val: &str) -> i32 {
    match str_val.find(ch) {
        Some(value) => value as i32,
        None => -1 as i32,
    }
}

//insert in a location
fn insert_in_location<'a>(pos: usize, ch: &str, main_str: &mut str) -> &'a str {
    let mut new_str = String::new();
    main_str.clone_into(&mut new_str);
    new_str.insert_str(pos, ch);
    &new_str
}
