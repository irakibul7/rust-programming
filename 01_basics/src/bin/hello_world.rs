// Hello World Example
// Run with: cargo run --bin hello_world

fn main() {
    // Basic printing
    println!("Hello, World!");

    // Formatted printing
    let name = "Rustacean";
    println!("Hello, {}!", name);  // {} is a placeholder for variables

    // Multiple value formatting
    let x = 5;
    let y = 10;
    println!("The sum of {} and {} is {}", x, y, x + y);
    
    println!("\nTry these exercises:");
    println!("1. Modify this program to ask for user's name");
    println!("2. Try different formatting options (padding, alignment)");
    println!("3. Print some calculations with different operators");
    
    println!("\nTry other examples:");
    println!("cargo run --bin variables");
} 