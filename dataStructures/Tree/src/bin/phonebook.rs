//goal : create phonebook using binary search tree
//left and right in vector: traversal is from left to right
//so first value is the left val and second val is the right val

use std::{cmp::Ordering, path::is_separator};

#[derive(Debug)]
struct Node {
    data: String,
    parent_index: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Phonebook {
    nodes: Vec<Node>,
    root: Option<usize>,
    filing_index: usize,
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
            filing_index: 0 as usize,
        }
    }

    //add the root
    //TODO: root either be the first letter which is a
    //TODO: root either be any letter

    fn create_root(&mut self, data: String) -> bool {
        //check if root already exist
        if let Some(idx) = self.root {
            //throw error
            println!("root already exist");
            return false;
        }

        //check if the given value is the A
        let char_val = data.chars().next();
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
                parent_index: None,
                children: vec![0, 0],
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
    fn add_numbers(&mut self, data: String) -> bool {
        // TODO: add the check for the root existence

        //compare nodeval string with the data string
        match self.nodes[self.filing_index].data.cmp(&data) {
            Ordering::Less => {
                //add the value to the node
                let node = Node {
                    data,
                    parent_index: Some(self.filing_index),
                    children: vec![0, 0],
                };
                let child_index = self.nodes.len();

                self.nodes.push(node);
                //update this in the parent node
                self.nodes[self.filing_index].children[0] = child_index;

                //update the filled index value, if chiild number reached two or more
                if self.nodes[self.filing_index].children.len() >= 2 {
                    self.filing_index += 1;
                }
                true
            }

            Ordering::Greater => {
                //add the value to the node
                let node = Node {
                    data,
                    parent_index: Some(self.filing_index),
                    children: vec![0, 0],
                };
                let child_index = self.nodes.len();

                self.nodes.push(node);
                //update this in the parent node

                self.nodes[self.filing_index].children[1] = child_index;

                //update the filled index value, if chiild number reached two or more
                if self.nodes[self.filing_index].children.len() >= 2 {
                    self.filing_index += 1;
                }

                true
                //add the value to the node
            }

            Ordering::Equal => {
                println!("value already exists");
                return false;
            }
        }
    }
}

fn phonebook_operations() {
    let root_vec = ["Alice", "Aaron", "Arthur"];
    let mut phonebook = Phonebook::create();

    //udpate the phonebook
    for val in root_vec {
        let is_root = phonebook.create_root(val.to_string());
        if is_root == true {
            println!("root found");
            break;
        }
    }

    println!("the phonebook is : {:#?}", phonebook);

    let names = [
        "Alice", "Brian", "Chloe", "David", "Emma", "Felix", "Grace", "Henry", "Isla", "Jack",
        "Kevin", "Liam", "Mia", "Noah", "Olivia", "Peter", "Quinn", "Rachel", "Sam", "Arthur",
    ];

    for name in names {
        phonebook.add_numbers(name.to_string());
    }

    println!("the phonebook after updating number : {}", phonebook);
}
fn main() {
    phonebook_operations();
}
