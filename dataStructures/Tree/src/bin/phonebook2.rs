//phonebook 2 using the avl tree in rust

#[path = "../utils.rs"]
mod utils;
use core::panic;
use std::cmp::Ordering;
use std::env::join_paths;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use utils::generate_random_name_and_number;

#[derive(Debug)]
struct PhoneData {
    name: String,
    call: u64,
}

#[derive(Debug)]
struct Node {
    data: PhoneData,
    parent_index: Option<usize>,
    height: isize,
    left_child: Option<usize>,
    right_child: Option<usize>,
}

#[derive(Debug)]
struct Phonebook {
    nodes: Vec<Node>,
    root: Option<usize>,
    find_time_counter: Option<usize>,
}

static PHONEBOOK_STORE: OnceLock<Mutex<Phonebook>> = OnceLock::new();

fn phonebook_store() -> &'static Mutex<Phonebook> {
    PHONEBOOK_STORE.get_or_init(|| Mutex::new(Phonebook::create()))
}

impl std::fmt::Display for Phonebook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the write! macro to format the output.
        // It works exactly like format! or println!
        write!(f, "{:#?}", self)
    }
}

impl Phonebook {
    fn create() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            find_time_counter: None,
        }
    }

    //add the root
    //TODO: root either be the first letter which is a
    //TODO: root either be any letter

    fn create_root(&mut self, data: PhoneData) -> bool {
        //check if root already exist
        if let Some(idx) = self.root {
            //throw error
            println!("root already exist");
            return false;
        }

        //check if the given value is the A
        let char_val = data.name.chars().next();
        let is_ascii = match char_val {
            Some(val) => {
                if val.is_ascii() {
                    true
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        };

        //the value of the ascii should be 65
        if is_ascii {
            //add the string to the root
            let node = Node {
                data,
                height: 0,
                parent_index: None,
                right_child: None,
                left_child: None,
            };

            let root_index = self.nodes.iter().len();

            //update the val in nodes
            self.nodes.push(node);
            self.root = Some(root_index);

            true
        } else {
            //can't create root
            println!("val is not a correct ascii value");
            return false;
        }
    }

    //this function decides itself who gonna be parent and
    fn add_numbers(&mut self, data: PhoneData) -> bool {
        // TODO: add the check for the root existencextern crate ;

        match self.root {
            Some(val) => {}
            None => {
                println!("root doesn't exists");
                return false;
            }
        }
        let mut current_idx = 0 as usize;
        let mut equal_val_counter = 0 as usize;

        loop {
            //compare nodeval string with the data string
            match self.nodes[current_idx].data.name.cmp(&data.name) {
                Ordering::Less => {
                    //add the value to the node
                    if let Some(val) = self.nodes[current_idx].right_child {
                        //update the current_idx to the indx we get
                        current_idx = val;
                    } else {
                        let node = Node {
                            data: data,
                            height: 0,
                            parent_index: Some(current_idx),
                            left_child: None,
                            right_child: None,
                        };

                        //upddate those value
                        let child_index = self.nodes.len();
                        self.nodes.push(node);

                        //update this in the parent node
                        self.nodes[current_idx].right_child = Some(child_index);

                        //height updation and balance factor check
                        let check_index = child_index;
                        let val = self.balance(check_index);
                        if val {
                            break true;
                        } else {
                            break false;
                        }
                    }
                }

                Ordering::Greater => {
                    //add the value to the node
                    if let Some(val) = self.nodes[current_idx].left_child {
                        //update the current_idx to the indx we get
                        current_idx = val;
                    } else {
                        let node = Node {
                            data: data,
                            height: 0,
                            parent_index: Some(current_idx),
                            left_child: None,
                            right_child: None,
                        };

                        //upddate those value
                        let child_index = self.nodes.len();
                        self.nodes.push(node);

                        //update this in the parent node
                        self.nodes[current_idx].left_child = Some(child_index);

                        let check_index = child_index;
                        let val = self.balance(check_index);
                        if val {
                            break true;
                        } else {
                            break false;
                        }
                    }
                }

                Ordering::Equal => {
                    println!("value already exists");
                    break false;
                }
            }
        }
    }

    fn find_numbers(&mut self, data: String) -> Option<u64> {
        // TODO: add the check for the root existence
        match self.root {
            Some(_) => {}
            None => {
                println!("root doesn't exists");
                return None;
            }
        }
        let start_time = Instant::now();
        let mut current_idx = 0 as usize;

        loop {
            //compare nodeval string with the data string
            match self.nodes[current_idx].data.name.cmp(&data) {
                Ordering::Less => {
                    //add the value to the node
                    if let Some(val) = self.nodes[current_idx].right_child {
                        current_idx = val;
                        self.find_time_counter = Some(self.find_time_counter.unwrap_or(0) + 1);
                    } else {
                        break None;
                    }
                }
                Ordering::Greater => {
                    //add the value to the node
                    if let Some(val) = self.nodes[current_idx].left_child {
                        //does right child is equal to the string
                        current_idx = val;
                        self.find_time_counter = Some(self.find_time_counter.unwrap_or(0) + 1);
                    } else {
                        break None;
                    }
                }

                Ordering::Equal => {
                    let result = self.nodes[current_idx].data.call;
                    let duration = start_time.elapsed();
                    println!("time taken : {:#?}", duration);

                    if let Some(counter) = self.find_time_counter {
                        println!("counts taken : {}", counter)
                    }

                    break Some(result);
                }
            }
        }
    }

    fn balance(&mut self, mut check_index: usize) -> bool {
        loop {
            if let Some(p_index) = self.nodes[check_index].parent_index {
                //update the height of the parent
                let mut left_child_height: isize = -1;
                let mut right_child_height: isize = -1;

                //get val for right child
                if let Some(left_child) = self.nodes[p_index].left_child {
                    left_child_height = self.nodes[left_child].height;
                }
                //get the left child
                if let Some(right_child) = self.nodes[p_index].right_child {
                    right_child_height = self.nodes[right_child].height;
                }

                let main_height = 1 + std::cmp::max(left_child_height, right_child_height);

                self.nodes[p_index].height = main_height;

                //leave balance factor for now
                let bal_factor = left_child_height - right_child_height;

                if bal_factor > 1 {}

                if bal_factor < -1 {}

                //update the check index
                check_index = p_index;
            } else {
                break true;
            }
        }
    }
}
fn create_root(data: PhoneData) -> bool {
    phonebook_store().lock().unwrap().create_root(data)
}

fn add_numbers(data: PhoneData) -> bool {
    phonebook_store().lock().unwrap().add_numbers(data)
}

fn find_numbers(name: String) -> Option<u64> {
    phonebook_store().lock().unwrap().find_numbers(name)
}

fn phonebook_operations() {
    let root_vec = [("Alice", "75708704233")];

    //udpate the phonebook
    for val in root_vec {
        let number: u64 = val.1.parse().unwrap();
        let phone_data = PhoneData {
            name: val.0.to_string(),
            call: number,
        };
        let is_root = create_root(phone_data);
        if is_root == true {
            println!("root found");
            break;
        }
    }

    println!(
        "the phonebook is : {:#?}",
        phonebook_store().lock().unwrap()
    );

    for _ in 0..10 {
        let (name, number_val) = generate_random_name_and_number();
        let phone_data = PhoneData {
            name,
            call: number_val,
        };

        //create the val
        let _ = add_numbers(phone_data);
    }

    println!(
        "the phonebook after adding numbers : {:#?}",
        phonebook_store().lock().unwrap()
    );

    // fn find_operation() {
    //     let find_string_vec = ["NPy43tUF8-DmCNs7", "NktmDwTIW03tIOy_", "NlkghAlr5K5AB4-E"];
    //     match find_numbers(find_string_vec[1].to_string()) {
    //         Some(val) => {
    //             println!("the number is : {}", val);
    //         }
    //         None => {
    //             panic!("number not found in the phonebook")
    //         }
    //     };
    // }
    //
    // std::thread::sleep(Duration::from_secs(7));
    // find_operation()
}

fn main() {
    phonebook_operations();
}
