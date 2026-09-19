//phonebook 2 using the avl tree in rust

#[path = "../utils.rs"]
mod utils;
use std::cmp::Ordering;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
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
    height: usize,
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

    fn height_of(&self, index: Option<usize>) -> usize {
        index.map_or(0, |index| self.nodes[index].height)
    }

    fn balance_factor(&self, index: usize) -> i64 {
        let node = &self.nodes[index];
        self.height_of(node.left_child) as i64 - self.height_of(node.right_child) as i64
    }

    fn refresh_height(&mut self, index: usize) {
        let (left, right) = (self.nodes[index].left_child, self.nodes[index].right_child);
        self.nodes[index].height = 1 + self.height_of(left).max(self.height_of(right));
    }

    fn rotate_right(&mut self, z: usize) {
        let parent = self.nodes[z].parent_index;
        let Some(y) = self.nodes[z].left_child else {
            return;
        };
        let middle_subtree = self.nodes[y].right_child;

        match parent {
            Some(parent) if self.nodes[parent].left_child == Some(z) => {
                self.nodes[parent].left_child = Some(y);
            }
            Some(parent) => {
                self.nodes[parent].right_child = Some(y);
            }
            None => self.root = Some(y),
        }

        self.nodes[z].left_child = middle_subtree;
        self.nodes[y].right_child = Some(z);
        self.nodes[y].parent_index = parent;
        self.nodes[z].parent_index = Some(y);
        if let Some(middle_subtree) = middle_subtree {
            self.nodes[middle_subtree].parent_index = Some(z);
        }

        self.refresh_height(z);
        self.refresh_height(y);
    }

    fn rotate_left(&mut self, z: usize) {
        let parent = self.nodes[z].parent_index;
        let Some(y) = self.nodes[z].right_child else {
            return;
        };
        let middle_subtree = self.nodes[y].left_child;

        match parent {
            Some(parent) if self.nodes[parent].right_child == Some(z) => {
                self.nodes[parent].right_child = Some(y);
            }
            Some(parent) => {
                self.nodes[parent].left_child = Some(y);
            }
            None => self.root = Some(y),
        }

        self.nodes[z].right_child = middle_subtree;
        self.nodes[y].left_child = Some(z);
        self.nodes[y].parent_index = parent;
        self.nodes[z].parent_index = Some(y);
        if let Some(middle_subtree) = middle_subtree {
            self.nodes[middle_subtree].parent_index = Some(z);
        }

        self.refresh_height(z);
        self.refresh_height(y);
    }

    fn rebalance_up(&mut self, start: usize) {
        let mut current = start;

        while let Some(parent) = self.nodes[current].parent_index {
            self.refresh_height(parent);
            let balance_factor = self.balance_factor(parent);

            if balance_factor > 1 {
                let left_child = self.nodes[parent]
                    .left_child
                    .expect("a left-heavy node must have a left child");
                if self.balance_factor(left_child) < 0 {
                    self.rotate_left(left_child);
                }
                self.rotate_right(parent);
                return;
            }

            if balance_factor < -1 {
                let right_child = self.nodes[parent]
                    .right_child
                    .expect("a right-heavy node must have a right child");
                if self.balance_factor(right_child) > 0 {
                    self.rotate_right(right_child);
                }
                self.rotate_left(parent);
                return;
            }

            current = parent;
        }
    }

    //add the root
    //TODO: root either be the first letter which is a
    //TODO: root either be any letter

    fn create_root(&mut self, data: PhoneData) -> bool {
        //check if root already exist
        if self.root.is_some() {
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
                height: 1,
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
        let mut current_idx = match self.root {
            Some(index) => index,
            None => {
                println!("root doesn't exists");
                return false;
            }
        };

        let (parent_index, add_to_left) = loop {
            match data.name.cmp(&self.nodes[current_idx].data.name) {
                Ordering::Less => match self.nodes[current_idx].left_child {
                    Some(left_child) => current_idx = left_child,
                    None => break (current_idx, true),
                },
                Ordering::Greater => match self.nodes[current_idx].right_child {
                    Some(right_child) => current_idx = right_child,
                    None => break (current_idx, false),
                },
                Ordering::Equal => {
                    println!("value already exists");
                    return false;
                }
            }
        };

        let child_index = self.nodes.len();
        self.nodes.push(Node {
            data,
            height: 1,
            parent_index: Some(parent_index),
            left_child: None,
            right_child: None,
        });

        if add_to_left {
            self.nodes[parent_index].left_child = Some(child_index);
        } else {
            self.nodes[parent_index].right_child = Some(child_index);
        }

        self.rebalance_up(child_index);
        true
    }

    fn find_numbers(&mut self, data: String) -> Option<u64> {
        // TODO: add the check for the root existence
        let mut current_idx = match self.root {
            Some(index) => index,
            None => {
                println!("root doesn't exists");
                return None;
            }
        };
        let start_time = Instant::now();
        self.find_time_counter = Some(0);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn phone_data(name: &str, call: u64) -> PhoneData {
        PhoneData {
            name: name.to_string(),
            call,
        }
    }

    fn phonebook_with_names(names: &[&str]) -> Phonebook {
        let mut phonebook = Phonebook::create();
        assert!(phonebook.create_root(phone_data(names[0], 0)));
        for (call, name) in names.iter().enumerate().skip(1) {
            assert!(phonebook.add_numbers(phone_data(name, call as u64)));
            assert_invariants(&phonebook);
        }
        phonebook
    }

    fn assert_invariants(phonebook: &Phonebook) {
        let root = phonebook.root.expect("a populated phonebook needs a root");
        assert_eq!(phonebook.nodes[root].parent_index, None);

        let mut reachable = vec![false; phonebook.nodes.len()];
        let mut stack = vec![root];
        while let Some(index) = stack.pop() {
            assert!(
                !reachable[index],
                "node {index} is reachable more than once"
            );
            reachable[index] = true;

            let node = &phonebook.nodes[index];
            for child in [node.left_child, node.right_child].into_iter().flatten() {
                assert_eq!(phonebook.nodes[child].parent_index, Some(index));
                stack.push(child);
            }

            let expected_height = 1 + phonebook
                .height_of(node.left_child)
                .max(phonebook.height_of(node.right_child));
            assert_eq!(node.height, expected_height, "stale height at {index}");
            assert!(
                phonebook.balance_factor(index).abs() <= 1,
                "node {index} is not AVL-balanced"
            );
        }
        assert!(reachable.into_iter().all(|is_reachable| is_reachable));

        fn collect_names<'a>(
            phonebook: &'a Phonebook,
            index: Option<usize>,
            names: &mut Vec<&'a str>,
        ) {
            if let Some(index) = index {
                collect_names(phonebook, phonebook.nodes[index].left_child, names);
                names.push(&phonebook.nodes[index].data.name);
                collect_names(phonebook, phonebook.nodes[index].right_child, names);
            }
        }

        let mut names = Vec::new();
        collect_names(phonebook, phonebook.root, &mut names);
        assert!(names.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn handles_all_four_rotation_cases() {
        for names in [
            ["C", "B", "A"], // LL
            ["A", "B", "C"], // RR
            ["C", "A", "B"], // LR
            ["A", "C", "B"], // RL
        ] {
            let phonebook = phonebook_with_names(&names);
            let root = phonebook.root.unwrap();
            assert_eq!(phonebook.nodes[root].data.name, "B");
            assert_eq!(phonebook.nodes[root].height, 2);
            assert_invariants(&phonebook);
        }
    }

    #[test]
    fn sorted_names_remain_balanced_and_searchable() {
        let names = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        let mut phonebook = phonebook_with_names(&names);

        assert_eq!(phonebook.nodes[phonebook.root.unwrap()].height, 4);
        assert_eq!(phonebook.find_numbers("O".to_string()), Some(14));
        assert_eq!(phonebook.find_numbers("missing".to_string()), None);
        assert_invariants(&phonebook);
    }

    #[test]
    fn duplicate_name_is_rejected() {
        let mut phonebook = phonebook_with_names(&["Alice"]);
        assert!(!phonebook.add_numbers(phone_data("Alice", 99)));
        assert_eq!(phonebook.nodes.len(), 1);
        assert_invariants(&phonebook);
    }
}
