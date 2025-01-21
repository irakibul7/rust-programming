# Rust Learning Journey

This repository contains multiple Rust projects organized by topics, from basics to advanced concepts.

## Project Structure

Each topic is a separate Cargo project in its own directory:

```
├── 01_basics/           # First steps in Rust
├── 02_ownership/        # Understanding ownership and borrowing
├── 03_structs_enums/    # Custom data types
├── 04_collections/      # Working with collections
├── 05_error_handling/   # Error handling patterns
├── 06_generics/        # Generic types and traits
├── 07_testing/         # Testing in Rust
├── 08_cli_projects/    # Command line applications
├── 09_web_projects/    # Web development
└── 10_advanced/        # Advanced topics
```

## How to Run

Each directory is a separate Cargo project. To run any project:

1. Navigate to the project directory:
   ```bash
   cd 01_basics
   ```

2. Run the project:
   ```bash
   cargo run
   ```

   Or run a specific example:
   ```bash
   cargo run --bin hello_world
   cargo run --bin variables
   ```

## Learning Path

Start with `01_basics` and progress through the numbered directories. Each project contains:
- Example code with detailed comments
- README with explanations
- Exercises to practice
- Additional resources for the topic

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) 