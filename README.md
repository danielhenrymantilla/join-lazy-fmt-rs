# join-lazy-fmt

> **Lazy `separator.join(iterable)` method and `lazy_format!` for Rust**

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)][Repository]
[![Latest version](https://img.shields.io/crates/v/join-lazy-fmt.svg)][crates.io]
[![Documentation](https://docs.rs/join-lazy-fmt/badge.svg)][Documentation]
[![License](https://img.shields.io/crates/l/join-lazy-fmt.svg)](https://github.com/danielhenrymantilla/join-lazy-fmt-rs#license)

## Usage

 1. Add the following line to your `Cargo.toml`, under `[dependencies]`

    ```toml
    join-lazy-fmt = "0.9.2"
    ```

 1. Add the folowing line to your `.rs` code to bring items in scope:

    ```rust
    use ::join_lazy_fmt::*;
    ```

## Example

```rust,edition2018
use ::join_lazy_fmt::*;

let sequence = format!("[{}]", ", ".join(0 .. 5));
assert_eq!(sequence, "[0, 1, 2, 3, 4]");

// Since `.join()` is lazy, this does not compute an infinite string.
let _ = ", ".join(0 ..);

const N: usize = 6;
let line = format!("+-{}-+", "-+-".join((1 .. N).map(|_| "---")));
// And the following allocates only one `String`:
let matrix = format!(
    "{line}\n{body}\n{line}\n",
    line=line,
    body="\n".join(
        (1 .. N).map(|i| lazy_format!(
            "| {row} |",
            row=" | ".join(
                (1 .. N).map(|j| lazy_format!(
                    "a{i}{j}",
                    i=i,
                    j=j,
                ))
            ),
        ))
    ),
);
assert_eq!(matrix, "\
+-----+-----+-----+-----+-----+
| a11 | a12 | a13 | a14 | a15 |
| a21 | a22 | a23 | a24 | a25 |
| a31 | a32 | a33 | a34 | a35 |
| a41 | a42 | a43 | a44 | a45 |
| a51 | a52 | a53 | a54 | a55 |
+-----+-----+-----+-----+-----+
");
```

[Repository]: https://github.com/danielhenrymantilla/join-lazy-fmt-rs
[Documentation]: https://docs.rs/join-lazy-fmt
[crates.io]: https://crates.io/crates/join-lazy-fmt
