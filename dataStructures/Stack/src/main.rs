//implementation for stack
//there are multiple ways to implement the stack

//implementation using array ( fixed size)
#[derive(Debug)]
pub struct Stack<T: Copy> {
    storage: Vec<T>,

    //capacity limits the size to which an storage grow
    capacity: usize,

    //index of the top most element in the storage
    top: usize,
}

pub enum PopResult {
    Data,
    Error,
}

impl<T: std::cmp::PartialEq + Copy> Stack<T> {
    //for new functions
    pub fn new(capacity: usize) -> Self {
        Self {
            storage: Vec::with_capacity(capacity),
            capacity,
            top: capacity - 1,
        }
    }

    //put element in the stack
    pub fn push(&mut self, data: T) -> bool {
        // 1.MANUAL CHECK : Enforce the strict capacity limit before pushing
        if self.storage.len() >= self.capacity {
            println!("Stack overflow");
            return false;
        }

        self.storage.push(data);

        //check if data is present in the vec
        if self.storage.contains(&data) {
            return true;
        }

        false
    }

    //get element from the stack
    pub fn pop(&mut self) -> T {
        match self.storage.pop() {
            Some(value) => {
                //update the top
                self.top -= 1;

                //return the popped value
                value
            }
            None => {
                panic!("underflow error")
            }
        }
    }
}
fn main() {
    let mut stack_space: Stack<i32> = Stack::new(9 as usize);

    println!("the stack space is : {:?}", stack_space);

    //push the data ( pushing data within capacity range )
    //need to watch when data is over the capacity range
    for val in vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110] {
        let response = stack_space.push(val);
        if !response {
            break;
        }
    }

    println!("the stack space after pushing elements : {:?}", stack_space);

    //popping element

    stack_space.pop();
    println!("the stack space after popping element : {:?}", stack_space);
}
