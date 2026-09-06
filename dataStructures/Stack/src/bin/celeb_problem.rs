// The Celebrity Problem
// Last Updated :
// 25 Aug, 2025
// Given a square matrix mat[][] of size n x n, where mat[i][j] == 1 means person i knows person j, and mat[i][j] == 0 means person i does not know person j, find the celebrity person where,
//
// A celebrity is defined as someone who:
//
// Is known by everyone else
// Does not know anyone (except themselves)
// Return the index of the celebrity if one exists, otherwise return -1.
//
// Note: It is guaranteed that mat[i][i] == 1 for all i
//
// Examples:
//
// Input: mat[][] = [[1, 1, 0],
//                              [0, 1, 0],
//                              [0, 1, 1]]
// Output: 1
// Explanation: 0th and 2nd person both know 1. Therefore, 1 is the celebrity.
//
// Input: mat[][] = [[1, 1],
//                              [1, 1]]
// Output: -1
// Explanation: The two people at the party both know each other. None of them is a celebrity.
//
// Input: mat[][] = [[1]]
// Output: 0

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

fn check_celeb() -> i32 {
    let matrix_maker = vec![[1, 1, 0], [0, 1, 0], [0, 1, 1]];

    let matrix_maker = vec![[1, 1], [1, 1]];
    let mut row_data: Option<usize> = None;

    for (row_index, iter_val) in matrix_maker.as_slice().iter().enumerate() {
        let mut count_in_row = 0;
        for (index, main_val) in iter_val.iter().enumerate() {
            if count_in_row == 0 && *main_val == 1 {
                count_in_row += 1;
            } else if count_in_row == 1 && *main_val == 1 {
                break;
            }

            if index == iter_val.len() - 1 && count_in_row == 1 {
                row_data = Some(row_index);
            }
        }
    }

    match row_data {
        Some(val) => {
            println!("the index value is : {}", val);
            let col_index = val; // FIXED: Removed "- 1"
            let mut col_val_count = 0;
            let total_rows = matrix_maker.len();

            for iter_val in matrix_maker.iter() {
                println!("the iter_val is : {}", iter_val[col_index]);
                if iter_val[col_index] == 1 {
                    col_val_count += 1;
                }
            }

            if col_val_count == total_rows {
                println!("is here");
                return val as i32;
            } else {
                println!("returning from else");
                -1
            }
        }
        None => {
            return -1;
        }
    }
}

fn main() {
    let res = check_celeb();
    println!("the response is :{}", res)
}
