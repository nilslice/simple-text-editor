#[test]
fn test_op_from_str() {
    use crate::ops::Operation;
    &[
        ("1 abc", Operation::Append("abc")),
        ("1  abc", Operation::Append(" abc")),
        ("1  ", Operation::Append(" ")),
        ("1 abc def ghi", Operation::Append("abc def ghi")),
        ("3 3", Operation::Print(3)),
        ("3          3", Operation::Print(3)),
        ("2 3", Operation::Delete(3)),
        ("2      3", Operation::Delete(3)),
        ("1      xy", Operation::Append("     xy")),
        ("4", Operation::Undo),
        ("5", Operation::Invalid),
        ("", Operation::Invalid),
        (" ", Operation::Invalid),
        ("    ", Operation::Invalid),
    ]
    .iter()
    .for_each(|case| {
        assert_eq!(Operation::from(case.0), case.1);
    });
}

#[test]
fn test_parse_input() {
    use crate::ops::Operation;
    let input = r#"8
    1 abc
    3 3
    2 3
    1 xy
    3 2
    4 
    4 
    3 1"#;
    let output = (
        8,
        vec![
            Operation::Append("abc"),
            Operation::Print(3),
            Operation::Delete(3),
            Operation::Append("xy"),
            Operation::Print(2),
            Operation::Undo,
            Operation::Undo,
            Operation::Print(1),
        ],
    );

    assert_eq!(crate::ops::parse(input).unwrap(), output);
}

#[test]
fn test_apply_ops() {
    // 8        # there are 8 operations to handle
    // 1 abc    # append "abc" to the buffer
    // 3 3      # print the character at position 3 (starting at 1)
    // 2 3      # delete 3 characters from the back of the buffer
    // 1 xy     # append "xy" to the buffer
    // 3 2      # print the character at position 2 (starting at 1)
    // 4        # undo the last append or delete operation (and remove it from history stack)
    // 4        # undo the last append or delete operation (and remove it from history stack)
    // 3 1      # print the character at position 1 (starting at 1)

    let (count, ops) = crate::ops::parse(
        r#"8
    1 abc
    3 3
    2 3
    1 xy
    3 2
    4 
    4 
    3 1"#,
    )
    .unwrap();
    assert_eq!(count, ops.len());

    let mut text = crate::text::Text::new("", count);
    text.apply(ops);
    assert_eq!("abc", text.output());
}
