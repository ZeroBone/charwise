# charwise
This lightweight, dependency-free rust library provides a convenient way to read characters from different resources.

For now, the following resources are supported:
* `std::fs::File`
* `std::io::Stdin`

This library is particularly useful when implementing handwritten [lexers](https://en.wikipedia.org/wiki/Lexical_analysis), because on one hand, we read characters one at a time in code and on the other hand, we may need the following features:

* Buffering of characters.
* Peeking next characters without consuming them.

All these features are implemented in `charwise`.

## Installation

In order to use `charwise`, simply add

```
charwise = "*"
```

to your `Cargo.toml` file.

## Example

```rust
use std::fs::File;
use charwise::Charwise;

fn main() {

    // file contains the following data: test content✌😜
    let file = File::open("test.txt").unwrap();

    let mut cwf = Charwise::from_file(file);

    assert_eq!('t', cwf.next().unwrap().unwrap());
    assert_eq!('e', cwf.next().unwrap().unwrap());
    assert_eq!('s', cwf.next().unwrap().unwrap());
    assert_eq!('t', cwf.next().unwrap().unwrap());
    assert_eq!(' ', cwf.next().unwrap().unwrap());
    assert_eq!('c', cwf.next().unwrap().unwrap());
    assert_eq!('o', cwf.next().unwrap().unwrap());

    // peek the next character without reading it
    assert_eq!('n', cwf.peek().unwrap().unwrap());

    assert_eq!('n', cwf.next().unwrap().unwrap());
    assert_eq!('t', cwf.next().unwrap().unwrap());

    // peek 4 characters ahead
    assert_eq!('✌', cwf.peek_nth(3).unwrap().unwrap());

    assert_eq!('e', cwf.next().unwrap().unwrap());
    assert_eq!('n', cwf.next().unwrap().unwrap());
    assert_eq!('t', cwf.next().unwrap().unwrap());
    assert_eq!('✌', cwf.next().unwrap().unwrap());
    assert_eq!('😜', cwf.next().unwrap().unwrap());

    // end of file
    assert!(cwf.next().is_none());

}
```