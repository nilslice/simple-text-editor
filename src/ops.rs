/// Contains all possible operations, as interpreted from commands via input. 
/// `Invalid` is a catch-all for any unrecognized commands.
#[derive(Clone, Debug, PartialEq)]
pub enum Operation<'a> {
    Append(&'a str),
    Delete(usize),
    Print(usize),
    Undo,
    Invalid,
}

/// Convert a line of input into an Operation.
impl<'a> From<&'a str> for Operation<'a> {
    fn from(input: &'a str) -> Self {  
        let input = input.trim_start(); 
        if input.is_empty() {
            return Operation::Invalid;
        }

        let mut chars = input.chars();
        match chars.next().map(|c| (c, chars.as_str())) {
            Some(('1', val)) => Operation::Append(remove_sep_space(val)),
            Some((op @ '2', val)) | Some((op @ '3', val)) => parse_delete_or_print(op, remove_sep_space(val)),
            Some(('4', _)) => Operation::Undo,
            _ => {
                Operation::Invalid
            }
        }
    }
}

/// Remove the whitespace between op and value, but not any leading whitespace of the value
fn remove_sep_space(val: &str) -> &str {
    let mut chars = val.chars();
    if let Some((_, rest)) = chars.next().map(|c| (c, chars.as_str())) {
        return rest;
    } else { 
        return val;
    }
}

/// Combine parsing logic from input when dealing with delete or print operations.
fn parse_delete_or_print(op: char, value_to_parse: &str) -> Operation {
    if let Ok(val) = value_to_parse.trim().parse::<usize>() {
        return match op {
            '2' => Operation::Delete(val),
            '3' => Operation::Print(val),
            _ => Operation::Invalid
        };
    } else {
        return Operation::Invalid
    }
}

/// Parse all operations from combined input, including the command count (first line).
pub fn parse(input: &str) -> Option<(usize, Vec<Operation>)>{
    let lines = input.lines();
    let num_ops = lines.clone().take(1).next().unwrap_or_default().parse();
    if num_ops.is_err() {
        return None
    }
    let ops = lines.skip(1).map(|line| { line.into() }).collect();
    Some((num_ops.unwrap(), ops))
}

/// Contains all possible operations which are eligible for "undo". 
/// `Append` and `Delete` maintain the inverse of their `Operation` type counterparts,
/// and hold the data necessary to undo the operation on the buffer.
#[derive(Debug)]
pub enum UndoableOperation {
    Append(usize),
    Delete(String),
}