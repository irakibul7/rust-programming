# Functions in Rust

A function is a reusable block of code designed to perform a specific task. Functions are fundamental building blocks in Rust and help organize and modularize your code.

The general syntax for defining a function in Rust is:

```rust
fn function_name() {
    // Code here
}
```

*   **Naming Convention:** Function names in Rust typically follow the snake case convention:
    *   All letters should be lowercase.
    *   Words should be separated by underscores (e.g., `my_function`, `calculate_sum`).

## Calling a Function

A function executes when it is *called* or *invoked*.

The syntax for invoking a function is:

```rust
fn main() {
    function_name(); // Calling the function
}
```

## Functions with Parameters

*Parameters* are variables or values declared within the function definition's parentheses. They act as placeholders for data that the function will receive when it's called.

```rust
fn function_name(param1: data_type, param2: data_type, ..., paramN: data_type) {
    // Function body; statements that use param1, param2, ..., paramN
}
```

*Arguments* are the actual values or variables passed to the function when it's invoked. They correspond to the parameters defined in the function signature.

```rust
function_name(argument1, argument2, ..., argumentN); // Calling the function with arguments
```

## Types of Arguments: Passing Data to Functions

Arguments can be passed to a function in two primary ways:

*   Pass by Value
*   Pass by Reference

### Arguments Passed by Value

When you pass an argument *by value*, a copy of the argument's value is made and passed to the corresponding parameter in the function. Any modifications made to the parameter within the function *do not* affect the original variable in the calling scope.

```rust
fn function_name(param1: data_type, ..., paramN: data_type) {
    // Function body; modifying param1, ..., paramN does NOT affect the original arguments.
}
```

**Example:**

```rust
fn square(mut n: i32) { // 'mut' allows modification within the function scope
    n = n * n;
    println!("The value of n inside function: {}", n);
}

fn main() {
    let n = 4;
    println!("The value of n before function call: {}", n);
    square(n); // Pass by value: A copy of 'n' is passed to 'square'
    println!("The value of n after function call: {}", n); // 'n' remains unchanged
}
```

Output:

```
The value of n before function call: 4
The value of n inside function: 16
The value of n after function call: 4
```

### Arguments Passed by Reference

When you pass an argument *by reference*, you're essentially providing the function with direct access to the original variable's memory location.  Any changes made to the parameter within the function *will* be reflected in the original variable in the calling scope. References allow you to modify the original data without creating a copy.  Use `&` to create a reference and `&mut` for a mutable reference.

```rust
fn function_name(param1: &mut data_type, ..., paramN: &mut data_type) {
    // Function body; modifying *param1, ..., *paramN WILL affect the original arguments.
}
```

**Example:**

```rust
fn square_pass_reference(n: &mut i32) {
    *n = *n * *n; // Dereference 'n' to access and modify the original value
    println!("The value of n inside function: {}", n);
}

fn main() {
    let mut n = 4; // 'mut' is required to create a mutable variable
    println!("The value of n before function call: {}", n);
    square_pass_reference(&mut n); // Pass by mutable reference
    println!("The value of n after function call: {}", n); // 'n' is modified!
}
```

Output:

```
The value of n before function call: 4
The value of n inside function: 16
The value of n after function call: 16
```

**Explanation:**

*   The `square_pass_reference` function takes a *mutable reference* (`&mut`) to an `i32`.  This means it receives a pointer to the memory location where the original variable `n` is stored.
*   `*n = *n * *n;`  The `*` operator is used to *dereference* the reference `n`. Dereferencing allows you to access and modify the value stored at the memory location pointed to by the reference.
*   The `main` function declares `n` as `mut n = 4;`. The `mut` keyword is *essential* because it makes the variable mutable, allowing it to be modified through a mutable reference.
*   When calling the function, `square_pass_reference(&mut n);`, `&mut n` creates a mutable reference to the variable `n` and passes it to the function.

**Note:**  When passing by reference and wanting to modify the value, you must first *dereference* the variable using the `*` operator.

### Multiple Return Values: Tuples

Rust allows you to return multiple values from a function using a *tuple*. A tuple is a fixed-size, ordered collection of elements of potentially different types.

**Syntax:**

```rust
fn function_name(param1: data_type, ..., paramN: data_type) -> (return_type1, return_type2, ..., return_typeN) {
    // Function body
    (value1, value2, ..., valueN) // Return a tuple
}
```

**Example:**

```rust
fn calculate_area_perimeter(length: i32, width: i32) -> (i32, i32) {
    let area = length * width;
    let perimeter = 2 * (length + width);
    (area, perimeter) // Return a tuple containing the area and perimeter
}

fn main() {
    let length = 4;
    let width = 3;
    let (area, perimeter) = calculate_area_perimeter(length, width); // Destructure the tuple
    println!("Area: {}, Perimeter: {}", area, perimeter);
}
```

## Functions with Arrays as Arguments

Arrays, which are fixed-size collections of elements of the same type, can be passed as arguments to functions.

### Pass by Value (Array Copy)

When an array is passed *by value*, a copy of the entire array is created and passed to the function. This means that any modifications made to the array within the function *will not* affect the original array.

```rust
fn function_name(mut array_name: [data_type; size]) {
    // Function body; modify 'array_name' (copy), original array is unchanged.
}
```

**Example:**

```rust
fn modify_my_array(mut arr: [i32; 5]) {
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function: {:?}", arr);
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    modify_my_array(arr); // Pass array by value (copy)
    println!("Array in main Function: {:?}", arr); // Original array is unchanged
}
```

Output:

```
Array in my Function: [1, 2, 8, 9, 5]
Array in main Function: [1, 2, 3, 4, 5]
```

### Pass by Reference (Array Modification)

When an array is passed *by reference*, the function receives direct access to the original array's memory. Any modifications made to the array within the function *will* affect the original array.

```rust
fn function_name(array_name: &mut [data_type; size]) {
    // Function body; modify 'array_name', original array IS changed.
}
```

**Example:**

```rust
fn modify_my_array_2(arr: &mut [i32; 5]) {
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function: {:?}", arr);
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5]; // 'mut' required for mutable reference
    modify_my_array_2(&mut arr); // Pass array by mutable reference
    println!("Array in main Function: {:?}", arr); // Original array IS changed
}
```

Output:

```
Array in my Function: [1, 2, 8, 9, 5]
Array in main Function: [1, 2, 8, 9, 5]
```

### What is Recursion?

Recursion is a programming technique where a function calls itself within its own definition during execution. It's a powerful way to solve problems that can be broken down into smaller, self-similar subproblems.

Some problems are naturally defined recursively. For example, the factorial of a non-negative integer *n* is defined as *n* times the factorial of *n-1*, with the base case being factorial(0) = 1.

```
factorial(n) = n * factorial(n-1)  for n > 0
factorial(0) = 1
```

### Parts of a Recursive Function

A recursive function must comprise two essential parts:

*   **Base Case:** The base case is a condition that stops the recursive calls. It defines a simple scenario where the function can directly return a value *without* making another recursive call. Without a base case, the function would call itself infinitely, leading to a stack overflow error.

*   **Recursive Case:** The recursive case is where the function calls itself with a modified input. This modified input must bring the problem closer to the base case. Each recursive call breaks down the original problem into a smaller, self-similar subproblem.

### Example: nth Fibonacci Number

The Fibonacci sequence is a series of numbers where each term is the sum of the two preceding terms:

```
1, 1, 2, 3, 5, 8, 13, 21, ...
```

Let's consider a recursive function to calculate the *n*th Fibonacci number. (Note: While elegant, a recursive Fibonacci implementation can be inefficient for large *n* due to repeated calculations. Iterative solutions are often preferred for performance.)

The recursive function has two base cases and one recursive case:

*   **Base Cases:**
    *   `fibonacci(0)` returns 0 (or 1, depending on the sequence's starting point).  This example uses 0.
    *   `fibonacci(1)` returns 1.

*   **Recursive Case:**
    *   `fibonacci(n)` returns `fibonacci(n-1) + fibonacci(n-2)` for `n > 1`. This is where the function calls itself twice to compute the Fibonacci number based on the two preceding terms.

**Example (Rust Code):**

```rust
fn fibonacci(term: i64) -> i64 {
    match term {
        0 => 0,  // Base case 1
        1 => 1,  // Base case 2
        _ => fibonacci(term-1) + fibonacci(term-2) // Recursive case
    }
}
```

