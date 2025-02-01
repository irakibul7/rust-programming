fn main() {
    println!("\n ===============================================");
    println!("🦀 Welcome to Conditional Expressions! 🦀");
    println!("================================================\n");

    println!("🔍 if let Expression");
    println!("================================================\n");
    
    // define a scrutinee expression    
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner","course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // if any of the value is matched, it can guess the value of the other variables
    if let (a, b, "course") = course {
        println!("a: {}, b: {}", a, b);
    }else {
        println!("Value unmatched");
    }

    println!("================================================\n");
    println!("🔍 Match Expression");
    println!("================================================\n");

    // match expression
    let course = ("Rust", "beginner","course");
    match course {
        ("Rust", "beginner","course") => println!("Wrote all values in pattern to be matched with the scrutinee expression"),
        _ => println!("Value unmatched"),
    }
    
    let x = 2;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }
}
