# Rust Basics

This project contains fundamental concepts of Rust programming language.

## Examples

1. Hello World (`cargo run --bin hello_world`)
   - Basic program structure
   - Printing to console
   - String formatting

2. Variables (`cargo run --bin variables`)
   - Variable declaration and mutability
   - Basic data types
   - Compound types
   - Type annotations

## How to Run

1. Make sure you're in the basics directory:
   ```bash
   cd 01_basics
   ```

2. Run a specific example:
   ```bash
   cargo run --bin hello_world
   cargo run --bin variables
   ```

   Or run the main program to see all available examples:
   ```bash
   cargo run
   ```

## Exercises

1. Modify the hello world program to:
   - Take user input using `std::io::stdin()`
   - Print a personalized greeting

2. Create a new program that:
   - Uses different numeric types
   - Performs basic arithmetic
   - Demonstrates type conversion

3. Practice with strings:
   - Create both `String` and `&str` types
   - Concatenate strings
   - Convert between types

## Additional Resources

- [Rust Book: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Rust Book: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust By Example: Basics](https://doc.rust-lang.org/rust-by-example/basic.html) 