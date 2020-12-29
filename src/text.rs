use crate::ops::*;

/// An arbitrary constraint limiting the number of operations that can be executed on a buffer.
const MAX_OPS: usize = 1_000_000;
/// An arbitrary constraint limiting the number of characters that can be deleted from a buffer.
const MAX_DELETE_OPS: usize = MAX_OPS*2;

/// Represents the buffer of characters which are operated on by the commands, and the stack of operations that are eligible for undo commands.
#[derive(Debug)]
pub struct Text {
    value: String,
    num_ops: usize,
    operation_stack: Vec<UndoableOperation>,
}

impl Text {
    /// Creates a new `Text` instance, which can start from an initial state instead of an empty buffer.
    pub fn new(init: impl AsRef<str>, num_ops: usize) -> Self {
        Self {
            value: init.as_ref().into(), 
            num_ops,
            operation_stack: vec![],
        }
    }

    /// Mutates the `Text` instance by executing each operation in `ops` serially.
    pub fn apply<'a>(&mut self, ops: Vec<Operation<'a>>) {
        assert!(ops.len() <= MAX_OPS && ops.len() == self.num_ops, "The input exceeds the max number of operations permitted. (1,000,000)");
        let mut delete_count = 0 as usize;
        ops.clone().into_iter().for_each(|op| { 
            return match op {
                Operation::Delete(n) => { 
                    delete_count += n; 
                }
                _ => {}
            }
        });
        assert!(delete_count <= MAX_DELETE_OPS, "The input exceeds the max number of characters which can be deleted. (2,000,000)");

        for op in ops {
            match op {
                Operation::Append(val) => {
                    self.value.push_str(val);
                    self.operation_stack.push(UndoableOperation::Append(val.len()));
                },
                Operation::Delete(n) if n <= self.value.len() => {
                    let mut deleted = vec![];
                    for _ in 0..n {
                        deleted.push(self.value.pop().unwrap() as u8);
                    }
                    
                    deleted.reverse();
                    self.operation_stack.push(UndoableOperation::Delete(String::from_utf8(deleted).unwrap()));
                },
                Operation::Print(i) if i <= self.value.len() => {
                    println!("{}", self.value.chars().nth(i-1).unwrap());
                }
                Operation::Undo => {
                    if let Some(op) = self.operation_stack.pop() {
                        match op {
                            UndoableOperation::Append(n) => {
                                for _ in 0..n {
                                    self.value.pop();
                                }
                            }
                            UndoableOperation::Delete(val) => {
                                self.value.push_str(&val);
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    /// Returns the current state of the `Text` instance's internal buffer.
    pub fn output(self) -> String {
        self.value
    }
}