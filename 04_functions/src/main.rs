fn main() {
    println!("\n ===============================================");
    println!("🦀 Welcome to Functions! 🦀");
    println!("================================================\n");

    display_message();
    println!("================================================\n");
    my_func(1,2);
    println!("================================================\n");

    let n = 4;
    println!("The value of n before function call: {}", n);
    println!("Invoke function");
    square(n);
    println!("\nThe value of n after function call: {}", n);
    println!("================================================\n");
    
    let mut n = 4;
    println!("The value of n before function call: {}", n);
    println!("Invoke function");
    square_pass_reference(&mut n);
    println!("\nThe value of n after function call: {}", n);
    println!("================================================\n");
    
}

fn display_message() {
    println!("This is user defined function");
}

fn my_func(param_1: i32, param_2: i32) {
    println!("The first value passed inside function: {}", param_1);
    println!("The second value passed inside function: {}", param_2);
}

fn square(mut n:i32){
    n = n * n;
    println!("The value of n inside function: {}", n);
}

fn square_pass_reference(n: &mut i32){
    *n = *n * *n;
    println!("The value of n inside function: {}", n);
}