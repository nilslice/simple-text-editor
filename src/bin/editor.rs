use simple_text_editor::{ops::*, text::*};
use std::io::{stdin, Read, Result};

fn main() -> Result<()> {
    // allocate a buffer to hold all raw command input, read all input to the buffer.
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;

    // parse the input into operations, filter out invalid operations which will not be run,
    // and apply the operations in serial onto the text node containing a mutable buffer.
    if let Some((n, ops)) = parse(&String::from_utf8(buf).expect("invalid input")) {
        let mut text = Text::new("", n);
        let valid_ops = ops
            .into_iter()
            .filter(|op| !matches!(op, Operation::Invalid))
            .collect::<Vec<Operation>>();
        text.apply(valid_ops);
        println!("{}", text.output());
    }

    Ok(())
}
