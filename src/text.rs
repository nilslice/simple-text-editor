use crate::ops::*;

/// An arbitrary constraint limiting the number of operations that can be executed on a buffer.
const MAX_OPS: usize = 1_000_000;
/// An arbitrary constraint limiting the number of characters that can be deleted from a buffer.
const MAX_DELETE_OPS: usize = MAX_OPS * 2;

/// Represents the buffer of characters which are operated on by the commands, and the stack of operations that are eligible for undo commands.
#[derive(Debug)]
pub struct Text {
    /// The underlying buffer which is mutated by the operations applied.
    value: String,
    // The count of operations as declared by the first line of input.
    num_ops: usize,
    // A stack of operations which are eligible for undo.
    operation_stack: Vec<UndoableOperation>,
}

/// ```
/// use simple_text_editor::ops;
/// use simple_text_editor::text::Text;
/// let (num_ops, ops) = ops::parse(r#"4
/// 1 hello
/// 2 1
/// 2 1
/// 1 p me!
/// "#).unwrap();
/// let mut text = Text::new("", num_ops);
/// text.apply(ops);
/// assert_eq!(text.output(), "help me!".to_string());
/// ```
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
    pub fn apply(&mut self, ops: Vec<Operation>) {
        assert!(
            ops.len() <= MAX_OPS,
            format!(
                "The input exceeds the max number of operations permitted. ({})",
                MAX_OPS
            )
        );
        assert!(
            ops.len() == self.num_ops,
            format!("The provided count doesn't match the number of valid operations parsed. (count = {}, valid operations = {})", self.num_ops, ops.len())
        );

        let mut delete_count = 0_usize;
        for op in ops {
            match op {
                // append the value to the inner buffer, and push an undoable operation to the operation stack,
                // along with the data needed to undo: the count of characters to remove from the buffer.
                Operation::Append(val) => {
                    self.value.push_str(val);
                    self.operation_stack
                        .push(UndoableOperation::Append(val.len()));
                }
                // delete `n` characters from the back of the buffer, and push an undoable operation to the
                // operation stack. keep the deleted characters (in order), and include them in the undoable
                // operation so they can be appended later in the undo operation.
                Operation::Delete(n) if n <= self.value.len() => {
                    delete_count += n;
                    assert!(
                        delete_count <= MAX_DELETE_OPS,
                        format!("The input exceeds the max number of characters which can be deleted. ({})", MAX_DELETE_OPS)
                    );

                    // keep the deleted characters so they can be re-appended in the case of an undo command
                    // NOTE: these characters are in reverse order as they were appended, and will need to
                    // be reversed before they are re-appended to the buffer.
                    let mut deleted = vec![];
                    for _ in 0..n {
                        deleted.push(self.value.pop().unwrap() as u8);
                    }

                    self.operation_stack
                        .push(UndoableOperation::Delete(deleted));
                }
                // print out the character at the 1-based index `i` from the buffer.
                Operation::Print(i) if i <= self.value.len() => {
                    println!("{}", self.value.chars().nth(i - 1).unwrap());
                }
                // undo a previous "append" or "delete" operation, maintained by the operation stack.
                // as a reversal of the previous affect, undoing an "append" is to delete the number
                // of appended characters from the back of the buffer, and undoing a "delete" is to
                // append the previously deleted characters to the back of the buffer. the operation
                // to be undone is the element popped from the operation stack.
                Operation::Undo => {
                    if let Some(op) = self.operation_stack.pop() {
                        match op {
                            UndoableOperation::Append(n) => {
                                for _ in 0..n {
                                    self.value.pop();
                                }
                            }
                            UndoableOperation::Delete(mut val) => {
                                // reverse the characters to put them in the correct append order,
                                // and push each back to the buffer. the characters are reversed
                                // here instead of in the Operation::Delete match arm so the work
                                // is deferred as there is no guarantee the undo operation will occur.
                                val.reverse();
                                val.iter().for_each(|c| self.value.push(*c as char));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Returns the current state of the `Text` instance's internal buffer.
    pub fn output(self) -> String {
        self.value
    }
}
