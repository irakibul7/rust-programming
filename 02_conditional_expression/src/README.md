# Conditional Expressions

A comprehensive guide to conditional expressions in Rust with examples and best practices. Ideal for those transitioning from other languages.

## Table of Contents

- [if Expression](#if-expression)
- [if let Expression](#if-let-expression)
- [match Expression](#match-expression)

There are three types of conditional expressions in Rust:

- `if` expression
- `if let` expression
- `match` expression

## if Expression

The `if` expression is used to execute a block of code if a condition is true.

There can be multiple `if` expressions in a single program.

```rust
if condition {
    // code to execute if condition is true
}
```

## if-else Expression

The `if-else` expression is used to execute a block of code if a condition is true, otherwise execute another block of code.

```rust
if condition {
    // code to execute if condition is true
} else {
    // code to execute if condition is false
}
``` 

## if-else if-else Expression

The `if-else if-else` expression is used to execute a block of code if a condition is true, otherwise execute another block of code.

```rust
if condition {
    // code to execute if condition is true
} else if condition {
    // code to execute if condition is true
} else {
    // code to execute if condition is false
}
```

### Nested if Expressions

You can nest `if` expressions inside other `if` expressions.

```rust
if condition {
    if condition {
        // code to execute if condition is true
    }
}
```

    Note: The nested if expression can also be written with a `&&` operator.

    ```rust
    if condition && condition {
        // code to execute if condition is true
    }
    ```

## if let Expression

The `if let` expression that allows pattern matching. The block of code is executed if the pattern matches.

```rust
if let (value1, value2) = match_expression {
    // code to execute if condition is true
}
```

## Match Expression

The `match` expression is used to match a value against a series of patterns.

```rust
match value {
    pattern => {
        // code to execute if pattern matches
    }
    _ => {
        // code to execute if pattern does not match
    }
}
```







