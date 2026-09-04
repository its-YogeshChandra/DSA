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
pub struct Node<T: Copy> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: Copy> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: std::cmp::PartialEq + std::fmt::Display + Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0 as usize,
        }
    }

    /// CREATE: Push to the front (LIFO). This element becomes index 0.
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1 as usize;
    }

    /// READ: Get value at a specific sequential index (0-based from head)
    pub fn get_at_index(&self, index: usize) -> Option<T> {
        let mut current = &self.head;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(node.data);
            }
            current = &node.next;
            current_index += 1;
        }
        None // Index out of bounds
    }

    /// UPDATE: Update value at a specific sequential index
    pub fn update_at_index(&mut self, index: usize, new_val: T) -> bool {
        let mut current = &mut self.head;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                node.data = new_val;
                return true;
            }
            current = &mut node.next;
            current_index += 1;
        }
        false // Index out of bounds
    }

    /// INSERT: Insert a new node at a specific sequential index
    pub fn insert_at_index(&mut self, index: usize, data: T) -> bool {
        // Special case: inserting at the head is just push_front
        if index == 0 {
            self.push_front(data);
            return true;
        }

        let mut current = &mut self.head;
        let mut current_index = 0;

        // Traverse to the node JUST BEFORE the target index
        while let Some(node) = current {
            if current_index == index - 1 {
                let new_node = Box::new(Node {
                    data,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return true;
            }
            current = &mut node.next;
            current_index += 1;
        }
        false // Index out of bounds
    }

    /// DELETE: Remove node at a specific sequential index
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        // Special case: deleting the head
        if index == 0 {
            match self.head.take() {
                Some(node) => {
                    self.head = node.next;
                    return true;
                }
                None => return false, // List is empty
            }
        }

        let mut current = &mut self.head;
        let mut current_index = 0;

        // Traverse to the node JUST BEFORE the target index
        while let Some(node) = current {
            if current_index == index - 1 {
                match node.next.take() {
                    Some(target_node) => {
                        // Bypass the target node
                        node.next = target_node.next;
                        return true;
                    }
                    None => return false, // Index out of bounds
                }
            }
            current = &mut node.next;
            current_index += 1;
        }
        false // Index out of bounds
    }

    /// READ: Check if an element is present anywhere in the list
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

    /// GET LENGTH: Count the number of nodes
    pub fn get_length(&self) -> usize {
        let mut current = &self.head;
        let mut count = 0;
        while let Some(_node) = current {
            count += 1;
            current = &current.as_ref().unwrap().next;
        }
        count
    }

    /// Helper to print the list with its implicit sequential indices for debugging
    pub fn print_with_indices(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        let mut index = 0;
        print!("List: ");
        while let Some(node) = current {
            print!("[{}: {}] ", index, node.data);
            current = &node.next;
            index += 1;
        }
        println!();
    }

    pub fn update_from_last(&mut self, nth_target_index: usize, data: T) -> bool {
        //check if the index is not out of bounds
        if nth_target_index >= self.length {
            return false;
        }

        let exact_index = self.length - 1 - nth_target_index;

        let response = self.update_at_index(exact_index, data);
        if !response {
            return false;
        }

        true
    }
}

fn main() {
    let mut linked_item: LinkedList<i32> = LinkedList::new();
    let entry_vec: Vec<i32> = vec![10, 20, 30, 40, 50];

    // Pushing 10, 20, 30, 40, 50 results in LIFO order:
    // Head -> 50 (idx 0) -> 40 (idx 1) -> 30 (idx 2) -> 20 (idx 3) -> 10 (idx 4)
    for val in entry_vec {
        linked_item.push_front(val);
    }

    println!("Initial list state:");
    linked_item.print_with_indices();

    // User wants to update the value at "index 2" (which is logically the 3rd item from the start/head)
    // In our LIFO list, index 2 is the value `30`.
    let target_index = 2;
    let new_value = 99;

    let success = linked_item.update_at_index(target_index, new_value);

    if success {
        println!(
            "Successfully updated index {} to {}",
            target_index, new_value
        );
    } else {
        println!("Failed to update: index {} is out of bounds.", target_index);
    }

    println!("\nList state after update:");
    linked_item.print_with_indices();

    // Let's also test insertion
    println!("\nInserting 77 at index 1...");
    linked_item.insert_at_index(1, 77);
    linked_item.print_with_indices();

    // And deletion
    println!("\nDeleting index 0 (the head)...");
    linked_item.delete_at_index(0);
    linked_item.print_with_indices();

    println!("\nFinal length: {}", linked_item.get_length());
}
