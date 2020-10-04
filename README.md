# Classic Computer Science Problems (in Rust)

This project contains selected problems from the book "Classical Computer Science Problems in Python" translated into Rust.

For comparison, the source code for CCSP in Python can be found [on Github](https://github.com/davecom/ClassicComputerScienceProblemsInPython).

There are a handful of goals:

- Demonstrate idiomatic Rust approaches to the problems found in the book
- Showcase differences between programming paradigms as represented by the two languages
- Minimize the use of external libraries that would do the "heavy lifting" for a solution

If you spot an error, typo, or better approach to a solution, feel free to reach out via creating an Issue or PR.

## Contributing Notes

As of this writing, I am by no means an expert in Rust - feel free to contribute!

We use `#[allow(dead_code)]` to suppress clippy warnings that the functions are never used. This warning happens because the functions are only ever used in tests.

### Handy Commands

For those who are forgetful or new, the following are used regularly:

```
cargo test
cargo fmt
cargo clippy
```