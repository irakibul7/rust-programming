// Declare our modules
mod basics {
    pub mod hello_world;
    pub mod variables;
}

fn main() {
    println!("Welcome to Rust Basics!");
    println!("\nAvailable examples:");
    println!("1. Hello World:");
    println!("   cargo run --bin hello_world");
    println!("\n2. Variables and Data Types:");
    println!("   cargo run --bin variables");
    
    println!("\nChoose an example to run using the commands above!");
    println!("\nTip: View the source code in src/bin/ to learn more about each example.");
}
