// Last Updated :
// 10 Sep, 2025
// Given a Singly Linked List, the task is to find the Length of the Linked List.
//
// Examples:
//
// Input: LinkedList = 1->3->1->2->1
// Output: 5
// Explanation: The linked list has 5 nodes.
//
// Input: LinkedList = 2->4->1->9->5->3->6
// Output: 7
// Explanation: The linked list has 7 nodes.
//
// Input: LinkedList = 10->20->30->40->50->60
// Output: 6
// Explanation: The linked list has 6 nodes.

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq + std::fmt::Display> LinkedList<T> {
    //return the created linked list

    //crud
    //create operation
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // push operation
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    //read
    pub fn contains(&self, target: &T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.data == target {
                return true;
            }
            current = &node.next;
        }
        false
    }

    //update find the value and replace it
    pub fn update(&mut self, target_val: &T, new_val: T) -> bool {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if &node.data == target_val {
                node.data = new_val;
                return true;
            }
            current = &mut node.next;
        }
        false
    }

    //delete : find the first occurence of value and delete it
    pub fn delete(&mut self, target: &T) -> bool {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return false, // End of list, target not found
                Some(node) if &node.data == target => {
                    // We found it. We bypass this node by replacing the current
                    // pointer with the node's `next` pointer.
                    *current = node.next.take();
                    return true;
                }
                Some(node) => {
                    // Move to the next node
                    current = &mut node.next;
                }
            }
        }
    }

    //iterate over the linked list and return an u32
    //gonna solve it using recursion
    pub fn get_length(&self) -> usize {
        let result = main_len(&self.head);
        result
    }

    //create method to fetch linked list
}

//helper function
//recursive function
fn main_len<T>(value: &Option<Box<Node<T>>>) -> usize {
    match value {
        Some(val) => 1 + main_len(&val.next),
        None => 0 as usize,
    }
}

//process to get length
fn get_length() {
    let mut linked_item: LinkedList<i32> = LinkedList::new();

    //update values in linked list
    let entry_vec: Vec<i32> = vec![10, 20, 30, 40, 50, 60];
    let entry_vec_len = entry_vec.len();

    for val in entry_vec {
        linked_item.push_front(val);
    }

    println!("the length of linked list : {}", linked_item.get_length());
    assert_eq!(entry_vec_len, linked_item.get_length())
}

fn main() {
    get_length();
}
