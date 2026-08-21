//linked list implementation
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

fn main() {
    println!("Hello, world!");
}
