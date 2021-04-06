# charwise
This lightweight, dependency-free rust library provides a convenient way to read characters from different resources.

For now, the following resources are supported:
* `std::fs::File`
* `std::io::Stdin`

This library is particularly useful when implementing handwritten [lexers](https://en.wikipedia.org/wiki/Lexical_analysis), because on one hand, we read characters one at a time in code and on the other hand, we may need the following features:

* Buffering of characters.
* Peeking next characters without consuming them.
* Abstracting from the source we are reading characters from.

All these features are implemented in `charwise`.

## Installation

In order to use `charwise`, simply add

```
charwise = "*"
```

to your `Cargo.toml` file.

## Example

(will be added shortly)