//binary tree in rust

#[derive(Debug)]
struct Node<T: std::fmt::Display> {
    data: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Binary_Tree<T: std::fmt::Display> {
    nodes: Vec<Node<T>>,
    root: Option<usize>,
}

impl<T: std::fmt::Display> Binary_Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    pub fn create_root(&mut self, data: T) {
        if let Some(root_index) = self.root {
            //don't create the value
            panic!("root already exists")
        }

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

        //binary tree parent node shouldn't have more then two chilren node
        let parent_length = self.nodes[parent_index].children.len();
        if parent_length >= 2 as usize {
            panic!["can add more value to the tree"]
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
