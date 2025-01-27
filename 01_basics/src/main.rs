// Declare our modules
mod basics {
    pub mod 1_hello_world;
    pub mod 2_variables_and_types;
}

fn main() {
    println!("Welcome to Rust Basics!");
    println!("\nAvailable examples:");
    println!("1. Hello World:");
    println!("   cargo run --bin 1_hello_world");
    println!("\n2. Variables and Data Types:");
    println!("   cargo run --bin 2_variables_and_types");
    
    println!("\nChoose an example to run using the commands above!");
    println!("\nTip: View the source code in src/bin/ to learn more about each example.");
}
