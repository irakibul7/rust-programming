# Loops in Rust

A comprehensive guide to loops in Rust with examples and best practices.

## Table of Contents

- [loop Expression](#loop-expression)
- [while Loop](#while-loop)
- [for Loop](#for-loop)
- [while let Loop](#while-let-loop)

There are two types of loops in Rust:

## Definite Loops

Loops in which the number of iterations is known at compile time.
Example - `for` loop

## Indefinite Loops

Loops in which the number of iterations is not known at compile time.

Example
- `loop` expression
- `while` loop

## Enumerate

To count how many times the loop has already executed, use the `.enumerate()` function.

```rust
for (index, value) in iterator.enumerate() {
    // Use index and value
}
```

## Continue Statement

The `continue` statement, when encountered inside a loop, skips the execution of the rest of the statements in the loop's body for the current iteration and returns the control to the start of the loop.




