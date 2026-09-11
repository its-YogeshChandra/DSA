//implement two stack in rs
// Implement two Stacks in an Array
// Last Updated :
// 20 Apr, 2026
// Create a data structure twoStacks that represent two stacks. Implementation of twoStacks should use only one array, i.e., both stacks should use the same array for storing elements.
//
// Following functions must be supported by twoStacks.
//
// push1(int x) --> pushes x to first stack
// push2(int x) --> pushes x to second stack
// pop1() --> pops an element from first stack and return the popped element
// pop2() --> pops an element from second stack and return the popped element
// Examples:
//
// Input: push1(2), push1(3), push2(4), pop1(), pop2(), pop2()
// Output: [3, 4, -1]
// Explanation: push1(2) the stack1 will be [2]
//                         push1(3) the stack1 will be [2,3]
//                         push2(4) the stack2 will be [4]
//                         pop1() the popped element will be 3 from stack1 and stack1 will be [2]
//                         pop2() the popped element will be 4 from stack2 and now stack2 is empty
//                         pop2() the stack2 is now empty hence returned -1
//
// Input: push1(1), push2(2), pop1(), push1(3), pop1(), pop1()
// Output: [1, 3, -1]
// Explanation: push1(1) the stack1 will be [1]
// push2(2) the stack2 will be [2]
// pop1() the popped element will be 1
// push1(3) the stack1 will be [3]
// pop1() the popped element will be 3
// pop1() the stack1 is now empty hence returned -1

#[derive(Debug)]
struct TwoStacks<T: Copy + std::fmt::Display> {
    storage: Vec<Option<T>>,
    capacity: usize,
    right_start_index: usize,
    left_start_index: usize,
    right_filled_index: Option<usize>,
    left_filled_index: Option<usize>,
}

impl<T: Copy + std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug> TwoStacks<T> {
    //create the storage unit
    fn create_store(capacity: usize) -> Self {
        let storage: Vec<Option<T>> = vec![None; capacity];

        Self {
            storage,
            capacity,
            left_start_index: 0,
            right_start_index: capacity - 1,
            right_filled_index: None,
            left_filled_index: None,
        }
    }

    fn push_through_1(&mut self, data: T) -> bool {
        //start filling the global storage array from the left side

        let pushing_index = match self.left_filled_index {
            Some(index) => {
                println!("the left filled index is : {}", index);
                index + 1
            }
            None => self.left_start_index,
        };

        match self.storage.get(pushing_index) {
            Some(val) => {
                //overflow error
                if let Some(val) = val {
                    println!("overflow left error")
                } else {
                    self.storage[pushing_index] = Some(data);

                    if let Some(val) = self.left_filled_index {
                        self.left_filled_index = Some(val + 1)
                    } else {
                        self.left_filled_index = Some(self.left_start_index)
                    }
                }
            }

            None => {
                panic!("storage missing space");
            }
        }

        if self.storage.contains(&Some(data)) == true {
            return true;
        }

        false
    }

    //push through 2
    fn push_through_2(&mut self, data: T) -> bool {
        //start filling the global storage array from the right side

        let pushing_index = match self.right_filled_index {
            Some(index) => {
                println!("the right filled index is : {}", index);
                index - 1
            }
            None => self.right_start_index,
        };

        match self.storage.get(pushing_index) {
            Some(val) => {
                //overflow error
                if let Some(val) = val {
                    println!("overflow right error");
                    return false;
                } else {
                    self.storage[pushing_index] = Some(data);

                    if let Some(val) = self.right_filled_index {
                        self.right_filled_index = Some(val - 1)
                    } else {
                        self.right_filled_index = Some(self.right_start_index)
                    }
                }
            }

            None => {
                panic!("storage missing space");
            }
        }

        if self.storage.contains(&Some(data)) == true {
            return true;
        }

        false
    }

    fn pop_through_1(&mut self) {
        //self.storage.pop()
    }

    fn pop_through_2(&mut self, data: T) {
        //self.storage.pop()
    }
}

fn twoStack_operations() {
    let mut global_storage: TwoStacks<u32> = TwoStacks::create_store(8);

    println!("global storage before push : {:#?}", global_storage);

    let val_vec: Vec<u32> = (0..=100).collect();
    for index_val in 0..8 as usize {
        //push the element in the global storage through different functions
        if index_val > 4 {
            global_storage.push_through_2(val_vec[index_val]);
        } else {
            global_storage.push_through_1(val_vec[index_val]);
        }
    }

    println!("--------------------------");
    println!("global storage after push : {:?}", global_storage);
    println!("--------------------------");
}

fn main() {
    twoStack_operations();
}
