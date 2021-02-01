/// Contains all possible operations, as interpreted from commands via input.
/// `Invalid` is a catch-all for any unrecognized commands.
#[derive(Clone, Debug, PartialEq)]
pub enum Operation<'a> {
    /// The append command, denotated by `1` in the program's input. The associated `&'a str`
    /// contains the string data to be appended to the buffer.
    /// ```
    /// use simple_text_editor::ops::*;
    /// let op = "1 append this text".into();
    /// match op {
    ///     Operation::Append(val) => {
    ///         assert_eq!(val, "append this text");
    ///     }
    ///     _ => panic!("should have matched Operation::Append"),
    /// }
    /// ```
    Append(&'a str),
    /// The delete command, denotated by `2` in the program's input. The associated `usize` is the
    /// number of characters to be delete from the back of the buffer.
    /// ```
    /// use simple_text_editor::ops::*;
    /// let op = "2 5".into();
    /// match op {
    ///     Operation::Delete(n) => {
    ///         assert_eq!(n, 5);
    ///     }
    ///     _ => panic!("should have matched Operation::Delete"),
    /// }
    /// ```
    Delete(usize),
    /// The print command, denotated by `3` in the program's input. The associated `usize` is the
    /// 1-based index at which the character from the buffer should be printed.
    /// ```
    /// use simple_text_editor::ops::*;
    /// let op = "3 1".into();
    /// match op {
    ///     Operation::Print(i) => {
    ///         assert_eq!(i, 1);
    ///     }
    ///     _ => panic!("should have matched Operation::Print"),
    /// }
    /// ```
    Print(usize),
    /// The undo command, denotated by `4` in the program's input. There is no associated data, and
    /// thus simply pops a command from a maintained stack of undo-eligible operations, either being
    /// append or delete.
    /// ```
    /// use simple_text_editor::ops::*;
    /// let op = "4".into();
    /// match op {
    ///     Operation::Undo => {
    ///         assert!(true);
    ///     }
    ///     _ => panic!("should have matched Operation::Undo"),
    /// }
    /// ```
    Undo,
    /// Invalid is a catch-all for any unrecognized commands, and is ignored by the program.
    /// ```
    /// use simple_text_editor::ops::*;
    /// let op = "__BADOPERATION__".into();
    /// match op {
    ///     Operation::Invalid => {
    ///         assert!(true);
    ///     }
    ///     _ => panic!("should have matched Operation::Invalid"),
    /// }
    /// ```
    Invalid,
}

/// Convert a line of input into an Operation.
/// ```
/// use simple_text_editor::ops::*;
/// let op = "1 abc".into();
/// assert!(matches!(op, Operation::Append("abc")));
/// ```
impl<'a> From<&'a str> for Operation<'a> {
    fn from(input: &'a str) -> Self {
        let input = input.trim_start();
        if input.is_empty() {
            return Operation::Invalid;
        }

        let mut chars = input.chars();
        match chars.next().map(|c| (c, chars.as_str())) {
            Some(('1', val)) => Operation::Append(remove_sep_space(val)),
            Some((op @ '2', val)) | Some((op @ '3', val)) => {
                parse_delete_or_print(op, remove_sep_space(val))
            }
            Some(('4', _)) => Operation::Undo,
            _ => Operation::Invalid,
        }
    }
}

/// Remove the whitespace between op and value, but not any leading whitespace of the value
fn remove_sep_space(val: &str) -> &str {
    let mut chars = val.chars();
    if let Some((_, rest)) = chars.next().map(|c| (c, chars.as_str())) {
        rest
    } else {
        val
    }
}

/// Combine parsing logic from input when dealing with delete or print operations.
fn parse_delete_or_print(op: char, value_to_parse: &str) -> Operation {
    if let Ok(val) = value_to_parse.trim().parse::<usize>() {
        match op {
            '2' => Operation::Delete(val),
            '3' => Operation::Print(val),
            _ => Operation::Invalid,
        }
    } else {
        Operation::Invalid
    }
}

/// Parse all operations from combined input, including the command count (first line).
/// ```
/// use simple_text_editor::ops::*;
/// assert_eq!(
///     parse(
///         r#"4
/// 1 abc
/// 2 1
/// 3 1
/// 4
/// "#
///     ),
///     Some((
///         4 as usize,
///         vec![
///             Operation::Append("abc"),
///             Operation::Delete(1),
///             Operation::Print(1),
///             Operation::Undo,
///         ]
///     ))
/// );
/// ```
pub fn parse(input: &str) -> Option<(usize, Vec<Operation>)> {
    let lines = input.lines();
    let num_ops = lines.clone().take(1).next().unwrap_or_default().parse();
    if num_ops.is_err() {
        return None;
    }
    let ops = lines.skip(1).map(|line| line.into()).collect();
    Some((num_ops.unwrap(), ops))
}

/// Contains all possible operations which are eligible for "undo".
/// `Append` and `Delete` maintain the inverse of their `Operation` type counterparts,
/// and hold the data necessary to undo the operation on the buffer.
#[derive(Debug)]
pub enum UndoableOperation {
    /// The undo operation for a previously executed append command. The associated `usize` is the
    /// count of characters which had been appended, and should now be deleted from the back of
    /// the buffer.
    Append(usize),
    /// The undo operation for a previously executed delete command. The associated `Vec<u8>` is the
    /// set of characters which had been previously popped from the back of the buffer, and should
    /// now be re-appended. Note, the characters are in reverse-order, which is the natural order
    /// after they had been popped from the buffer. As an optimization, the program will lazily
    /// re-order these to be pushed onto the buffer so they are accurately re-appended.
    Delete(Vec<u8>),
}
