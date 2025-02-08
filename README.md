# Rust Learning Journey

This repository is designed to guide you through learning Rust, from the fundamentals to more advanced concepts. It's structured as a series of Cargo projects, each focusing on a specific topic.

## Project Structure

The repository is organized into directories, with each directory representing a distinct Rust learning module.  The naming convention uses sequential numbers to suggest a learning path.

```
├── 01_basics/                   # First steps in Rust: Hello World, variables, data types, basic operations
├── 02_conditional_expression/   # Conditional statements: if/else, match expressions
├── 03_loops/                   # Iteration: for, while, loop keywords, iterators
├── 04_functions/                 # Functions: definitions, parameters, return values, closures, recursion
├── 05_error_handling/          # Error handling: Result, Option, panic!, error propagation
├── 06_generics/                  # Generic types and traits: writing reusable code
├── 07_testing/                   # Testing in Rust: unit tests, integration tests, documentation tests
├── 08_cli_projects/             # Command-line applications: building interactive tools
├── 09_web_projects/             # Web development: simple web servers, routing (potential for future expansion)
└── 10_advanced/                # Advanced topics: concurrency, unsafe Rust, macros (potential for future topics)
```

## Getting Started

1.  **Clone the Repository:** Clone this repository to your local machine:

    ```bash
    git clone <repository_url>
    cd rust-programming
    ```

2.  **Install Rust:** If you don't already have Rust installed, follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Running the Projects

Each directory contains a complete Cargo project. To run a project:

1.  **Navigate to the Project Directory:** Use the `cd` command to change your current directory to the desired project:

    ```bash
    cd 01_basics
    ```

2.  **Run the Project:** Use the `cargo run` command to build and execute the project:

    ```bash
    cargo run
    ```

3.  **Running Specific Examples:** Run a specific binary using the `--bin` flag:

    ```bash
    cargo run --bin hello_world  # Run the 'hello_world' example
    cargo run --bin variables    # Run the 'variables' example
    ```

## Learning Path and Project Structure

It's recommended to start with the `01_basics` directory and progress sequentially through the numbered modules. Each project directory typically contains the following:

*   **Example Code:** Rust source code (`.rs` files) demonstrating the concepts covered in the module. Code is commented extensively to explain each part.
*   **`README.md`:** A Markdown file providing a detailed explanation of the topic, code examples, and links to relevant resources. This file!
*   **Exercises (Optional):**  Some modules may include exercises to help you practice and solidify your understanding of the concepts.  Look for a directory like `exercises` or files with names like `exercise.rs`.
*   **Additional Resources (Links):** Links to external resources (books, articles, documentation) to further expand your knowledge on the topic.

## Contributing

Contributions are welcome! If you find errors, have suggestions for improvements, or want to add new content, please submit a pull request.  Follow the contribution guidelines in the `CONTRIBUTING.md` file

## Resources

These resources can be invaluable in your Rust learning journey:

*   **The Rust Programming Language Book ("The Book"):** [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/) - The official and comprehensive guide to learning Rust.
*   **Rust by Example:** [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/) - A collection of runnable examples that illustrate various Rust concepts.
*   **Rust Standard Library Documentation:** [https://doc.rust-lang.org/std/](https://doc.rust-lang.org/std/) - The official documentation for the Rust standard library.
*   **The Cargo Guide:** [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/) - Documentation for Cargo, Rust's build system and package manager.
*   **Awesome Rust:** [https://github.com/rust-unofficial/awesome-rust](https://github.com/rust-unofficial/awesome-rust) - A curated list of Rust libraries, tools, and resources.

Good luck on your Rust adventure!
