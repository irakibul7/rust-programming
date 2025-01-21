// Variables and Data Types Example
// Run with: cargo run --bin variables

fn main() {
    println!("Learning Rust Variables and Data Types!\n");

    // SECTION 1: Variable Basics
    println!("--- Variable Basics ---");
    
    // Variables are immutable by default
    let immutable_number = 42;
    println!("Immutable number: {}", immutable_number);
    
    // To make a variable mutable, use 'mut'
    let mut mutable_number = 42;
    println!("Original mutable number: {}", mutable_number);
    mutable_number = 43;
    println!("Changed mutable number: {}", mutable_number);

    // Constants require type annotation and are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("Constant value: {}", MAX_POINTS);

    // SECTION 2: Basic Types
    println!("\n--- Basic Types ---");

    // Integer types
    let integer: i32 = -42;  // 32-bit signed integer
    let unsigned: u32 = 42;  // 32-bit unsigned integer
    println!("Integers - Signed: {}, Unsigned: {}", integer, unsigned);

    // Floating-point types
    let float: f64 = 3.14;  // 64-bit float
    println!("Float: {}", float);

    // Boolean type
    let is_active: bool = true;
    println!("Boolean: {}", is_active);

    // Character type (Unicode scalar value)
    let letter: char = 'A';
    println!("Character: {}", letter);

    // SECTION 3: Compound Types
    println!("\n--- Compound Types ---");

    // String types
    let text: &str = "Hello, Rust!";  // String slice
    let string: String = String::from("Hello, Rust!");  // Owned string
    println!("String slice: {}", text);
    println!("Owned string: {}", string);

    // Tuple type
    let tuple: (i32, f64, &str) = (1, 3.14, "tuple");
    println!("Tuple: {:?}", tuple);  // {:?} enables debug printing
    println!("Accessing tuple - First value: {}", tuple.0);

    // Array type (fixed length)
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("Accessing array - First element: {}", array[0]);

    // SECTION 4: Type Inference
    println!("\n--- Type Inference ---");
    let inferred_int = 42;  // Rust infers i32
    let inferred_float = 3.14;  // Rust infers f64
    println!("Inferred types - Int: {}, Float: {}", inferred_int, inferred_float);

    println!("\nTry these exercises:");
    println!("1. Create variables of different integer types (i8, i16, i64, etc.)");
    println!("2. Try to modify an immutable variable (notice the error)");
    println!("3. Create an array with a different size and type");
    println!("4. Practice string concatenation and manipulation");
} 