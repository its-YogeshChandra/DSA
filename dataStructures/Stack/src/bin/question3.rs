// Infix to Postfix Expression
// Last Updated :
// 15 Sep, 2025
// Given a string s representing an infix expression ("operand1 operator operand2" ), Convert it into its postfix notation ("operand1 operand2 operator").
//
// Note: The precedence order is as follows: (^) has the highest precedence and is evaluated from right to left, (* and /) come next with left to right associativity, and (+ and -) have the lowest precedence with left to right associativity.
//
// Examples:
//
// Input: s = "a*(b+c)/d"
// Output: abc+*d/
// Explanation: The expression is a * (b + c) / d. First, inside the brackets, b + c becomes bc+. Now the expression looks like a * (bc+) / d. Next, multiply a with (bc+), so it becomes abc+* . Finally, divide this result by d, so it becomes abc+*d/.
//
// Input: s = "a+b*c+d"
// Output: abc*+d+
// Explanation: The expression a + b * c + d is converted by first doing b * c → bc*, then adding a → abc*+, and finally adding d → abc*+d

//An infix expression is a mathematical format where the operator is placed between its operands

//In a postfix expression, also called Reverse Polish Notation (RPN), the operator is placed after its operands.

#[derive(Debug)]
pub struct Stack<T: Copy> {
    storage: Vec<T>,

    //capacity limits the size to which an storage grow
    capacity: usize,

    //index of the top most element in the storage
    top: usize,
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

fn infix_to_postfix(infix_str: String) {
    //read the infix string
    let char_vec: Vec<char> = infix_str.chars().collect();
    for (index, char) in char_vec.iter().enumerate() {
        //check val and put the val in postfix order
    }
}

fn main() {
    todo!();
}
