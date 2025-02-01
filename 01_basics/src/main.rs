// Declare our modules
mod basics {
    pub mod 1_hello_world;
    pub mod 2_variables_and_types;
    pub mod 3_operators;
}

fn main() {
    println!("\n=================================");
    println!("🦀 Welcome to Rust Basics! 🦀");
    println!("=================================\n");
    
    println!("📚 Available Examples:\n");
    
    println!("1️⃣  Hello World");
    println!("   Run: cargo run --bin 1_hello_world");
    
    println!("\n2️⃣  Variables and Data Types"); 
    println!("   Run: cargo run --bin 2_variables_and_types");
    
    println!("\n3️⃣  Operators");
    println!("   Run: cargo run --bin 3_operators");
    
    println!("\n📝 Instructions:");
    println!("- Choose an example by running its command");
    println!("- View source code in src/bin/ directory");
    println!("- Each example includes detailed comments\n");
    
    println!("=================================");
}
