# Strings in Rust

Strings in Rust represent a sequence of Unicode characters. Unlike some other languages, Rust strings are not null-terminated and can contain null characters.

## String Types: `&str` and `String`

Rust provides two main types for working with strings:

*   **`&str` (String Slice):** A primitive type representing an immutable view into a sequence of UTF-8 encoded bytes.
*   **`String`:** A growable, mutable, owned string type stored as a UTF-8 encoded vector.

### String Literal (`&str`)

*   Primitive type
*   Immutable
*   Fixed-length, stored in memory
*   Value known at compile time

**Example:**

```rust
let language: &str = "Rust";
```

### `String` Object

A `String` object provides a growable, mutable buffer of UTF-8 encoded bytes.

#### Creating an Empty `String`

```rust
let language = String::new();
let s_language = language.to_string(); // Converts an empty String to String object
```

#### Creating an Initialized `String`

Using the `String::from()` method:

```rust
let language = String::from("Rust");
```

Converting a string literal to a `String`:

```rust
let language: &str = "Rust";
let s_language = language.to_string();
```

## Core `String` Methods

Here are some of the most commonly used built-in functions for manipulating `String` objects. Refer to the [Rust documentation](https://doc.rust-lang.org/std/string/struct.String.html) for a complete list.

### Capacity in Bytes

The `capacity()` method returns the number of bytes allocated for the `String`, while `len()` gives the number of bytes currently used.

**Syntax:**

```rust
let my_string = String::from("Hello");
let capacity = my_string.capacity();
let length = my_string.len();

println!("Capacity: {}, Length: {}", capacity, length);
```

**Note:** The length of a `String` will always be less than or equal to its capacity.

### Finding a Substring

Use the `contains()` method to check if a `String` contains a specific substring.

**Syntax:**

```rust
let my_string = String::from("Hello, World!");
let contains_world = my_string.contains("World");

println!("Contains 'World': {}", contains_world);
```

### Replacing a Substring

The `replace()` method replaces all occurrences of one substring with another.

**Syntax:**

```rust
let my_string = String::from("Hello, World!");
let new_string = my_string.replace("World", "Rust");

println!("Original: {}, Replaced: {}", my_string, new_string);
```

### Trimming Whitespace

The `trim()` method removes leading and trailing whitespace from a `String`.

**Syntax:**

```rust
let my_string = String::from("   Hello, World!   ");
let trimmed_string = my_string.trim();

println!("Original: '{}', Trimmed: '{}'", my_string, trimmed_string);
```

**Note:** `trim()` only removes whitespace from the beginning and end of the string.

## Iterating Over Strings

### Tokenizing a String

#### Splitting on Whitespace

The `split_whitespace()` method splits a `String` into substrings separated by whitespace.

**Syntax:**

```rust
let my_string = String::from("Hello World Rust");

for word in my_string.split_whitespace() {
    println!("{}", word);
}
```

#### Splitting on a Custom Character

The `split()` method splits a `String` into substrings based on a specified delimiter.

**Syntax:**

```rust
let my_string = String::from("apple,banana,orange");

for fruit in my_string.split(",") {
    println!("{}", fruit);
}
```

### Iterating Over Characters

The `chars()` method iterates over each character in a `String`.

**Syntax:**

```rust
let my_string = String::from("Rust");

for character in my_string.chars() {
    println!("{}", character);
}
```

## Modifying Strings

### Appending to a `String`

#### Using `push_str()`

Use the `push_str()` method to append a string slice to a mutable `String`.

```rust
let mut my_string = String::from("Hello");
my_string.push_str(", World!");

println!("{}", my_string);
```

### Concatenation

#### Using the `+` Operator

A `String` can be concatenated with a string slice using the `+` operator. Note that the right-hand-side operand must be borrowed.

```rust
let string1 = String::from("Hello, ");
let string2 = "World!";
let concatenated_string = string1 + string2;

println!("{}", concatenated_string);
```

#### Using the `format!` Macro

The `format!` macro creates a new `String` by formatting values.

```rust
let name = String::from("Alice");
let age = 30;
let formatted_string = format!("Name: {}, Age: {}", name, age);

println!("{}", formatted_string);

// Example with order specified
let formatted_string_with_order = format!("Age: {1}, Name: {0}", name, age);
println!("{}", formatted_string_with_order);
```

## Slicing a String

Slicing creates a new `&str` (string slice) that refers to a portion of an existing `String` or `&str`.

**Syntax:**

```rust
let my_string = String::from("Hello, World!");
let slice = &my_string[0..5]; // "Hello"

println!("{}", slice);
```

**Note:**

*   `start_index` is inclusive, `end_index` is exclusive.
*   `&` indicates that the slice borrows the string.
*   `start_index` must be between `0` and the length of the string.
```
