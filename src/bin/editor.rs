use std::io::Read;
use simple_text_editor::{ops::*, text::*};

fn main() -> std::io::Result<()> {
    let mut buf = vec![];
    let mut input = std::io::stdin();
    input.read_to_end(&mut buf)?;
    if let Some((n, ops)) = parse(&String::from_utf8(buf).expect("invalid input")) {
        let mut text = Text::new("", n);
        text.apply(ops);
        println!("{}", text.output());
    }

    Ok(())
}