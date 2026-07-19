//linked list implementation
//doubly linked list implementation

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

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
}

fn main() {
    let mut x: Option<i32> = None;
    let y = x.take();

    println!("the x is : {:#?}", x);
    println!("the x is : {:#?}", y);
}
