# Simple Text Editor

[![crates.io](https://img.shields.io/crates/v/simple-text-editor.svg)](https://crates.io/crates/simple-text-editor)
[![docs.rs](https://docs.rs/simple-text-editor/badge.svg)](https://docs.rs/simple-text-editor)

This program implements a basic text editing protocol in which the following commands can be executed:

- `1 x`, where `1` is the command to append text to a buffer, and `x` is the arbitrary-length set of characters to append.
- `2 n`, where `2` is the command to delete text from a buffer, and `n` is the number of characters from the back of the buffer to delete.
- `3 i`, where `3` is the command to print a character from the buffer, and `i` is the 1-based index position of the character to print.
- `4`, where `4` is the command to undo a previously executed command (only affectinf commands `1` or `2`).

> **Note:** The protocol specifies that the first line of the input read by the program is interpreted as the number of commands that follow.

## Usage

To test this, build the code using the `cargo` toolchain, invoking the Rust compiler. _To install `cargo` and Rust, see: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)_

```sh
$ cargo build --bin editor
```

Then, use the provided test input file `input.txt`, or create your own.

```sh 
$ cat input.txt | ./target/debug/editor
```

This should print both the characters (one per line, per "print" command, e.g. `3`) as well as the final buffer after all commands have exectuted.

## Contributions

This is just for fun! However, as someone who is always trying to improve their programming skills, I would be grateful to know where optimizations could be made. PRs are welcome. Or if a bug is encountered, file an issue and I'll take a look!