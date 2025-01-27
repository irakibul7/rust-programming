# Hello World Program

The main function is the entry point of the program. It is the first function that is called when the program is run.


The line `fn main() {` declares a function named main that takes no arguments and returns nothing.

The line `} `closes the main function.

The line `println!` is a macro that prints a string to the console.

The line `let name = "Rustacean";` declares a variable named name and assigns it the value "Rustacean". This line ends with a semicolon(;) which is used to indicate the end of a statement.

The line `print!("Hello, {}!", name);` prints the string "Hello, Rustacean!" to the console.

## What is a macro?

A macro is an expression that has a `!` before the parenthesis (). i.e.
macro_name!()

## What are macros used for?

Macros are used to generate code at compile time. They are used to create functions, structs, enums, and other code constructs.

# The Basic Formatting Rules

- Rust code blocks are enclosed in curly braces `{}`.
- Statements are terminated by a semicolon `;`.
- Expressions do not end with a semicolon.
- Functions are defined using the `fn` keyword.
- Variables are declared using the `let` keyword.
- Comments are enclosed in `//`.

## Single Placeholder

The single placeholder is used to format a single value in a string.

```rust
let name = "Rustacean";
println!("Hello, {}!", name);
```

## Multiple Placeholders

The multiple placeholders are used to format multiple values in a string.

```rust
let name = "Rustacean";
let age = 25;
println!("Hello, {}! You are {} years old.", name, age); // Hello, Rustacean! You are 25 years old.
```

## Positional Arguments

The positional arguments specify the position of the value in the string.

The first value is assigned 0, the second value is assigned 1, and so on.

```rust
let name = "Rustacean";
let age = 25;
println!("{1} is {0} years old.", age, name); // Rustacean is 25 years old.
```

## Named Arguments

The named arguments specify the name of the value in the string.

```rust
let name = "Rustacean";
let age = 25;
println!("{name} is {age} years old."); // Rustacean is 25 years old.
```

## Placeholder Traits

If we want to convert the value to binary, hexadecimal, or octal write:

`{:b}` for binary
`{:x}` for hexadecimal
`{:o}` for octal

```rust
let num = 10;
println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", num, num, num); // Binary: 1010, Hexadecimal: a, Octal: 12
```

## Debug Trait

It is possible to print multiple values using a single placeholder with the debug trait.

```rust
let name = "Rustacean";
let age = 25;
println!("{:?}", (name, age)); // ("Rustacean", 25)
``` 

# Printing Styles

The table below shows the different macros used to print in Rust.

| Macro | Description |
|-------|-------------|
| `println!` | Prints a string to the console. |
| `print!` | Prints a string to the console without a newline. |
| `eprintln!` | Prints an error message to the console with a newline. |
| `eprint!` | Prints an error message to the console without a newline. |


# Comments

Comments are used to add notes to the code. They are not compiled and are ignored by the compiler.

## Line Comments

Line comments are used to add comments to a single line of code.

```rust
let x = 5; // This is a line comment.
```

## Block Comments

Block comments are used to add comments to multiple lines of code.

```rust
/* This is a block comment.
This is a block comment.
This is a block comment.
*/
```

## Doc Comments

Doc comments are used to add documentation to the code. They are used to describe the code and are used by the compiler to generate documentation.

```rust
/// This is a doc comment.
```

