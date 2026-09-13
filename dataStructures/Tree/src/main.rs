//Trees
//implementation of the tree

use core::panic;

//parent child relationship

#[derive(Debug)]
struct Node<T: std::fmt::Display> {
    data: T,
    parent: Option<usize>,
    children: Vec<usize>,
}
#[derive(Debug)]
struct Tree<T: std::fmt::Display> {
    nodes: Vec<Node<T>>,
    root: Option<usize>,
}

impl<T: std::fmt::Display> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    pub fn create_root(&mut self, data: T) {
        // TODO: !check so that multiple roots can't be created

        let node = Node {
            data,
            parent: None,
            children: Vec::new(),
        };

        //update in the node
        let root_index = self.nodes.len();
        self.nodes.push(node);
        self.root = Some(root_index);
    }

    fn create_child(&mut self, data: T, parent_index: usize) {
        //to prevent overflow
        if parent_index >= self.nodes.len() {
            panic!("index overflow: reason parent index")
        }
        let node = Node {
            data,
            parent: Some(parent_index),
            children: Vec::new(),
        };

        //udpdate the child in the parent node
        let child_index = self.nodes.len();
        self.nodes.push(node);

        //update the parent node
        self.nodes[parent_index].children.push(child_index);
    }
}

fn tree_operations() {
    //call teh create root function
    let mut tree_storage: Tree<i32> = Tree::new();

    //create root
    tree_storage.create_root(10);
    println!("tree is : {:#?}", tree_storage);

    //add the chidren to the tree tree_operations
    for idx in 0..6 {
        tree_storage.create_child(23, idx as usize);
    }

    println!("tree is : {:#?}", tree_storage);
}
fn main() {
    //call the function
    tree_operations();
}
