fn main() {
    println!("\n ===============================================");
    println!("🦀 Welcome to Loops! 🦀");
    println!("================================================\n");

    println!("🔍 For loops");
    println!("================================================\n");

    for i in 0..5 {
        println!("{}", i);
    }


    // To count how many times the loop has already executed
    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }

    println!("================================================\n");
    println!("🔍 While loops");
    println!("================================================\n");

    let mut var = 1;
    let threshold = 5;
    let mut found = false;

    while !found {
        var += 1;
        println!("{}", var);
        if var > threshold && var % 2 == 0 {
            found = true;
        }
    }

    println!("\n================================================");
    println!("🔍 Break Statement");
    println!("================================================\n");

    for i in 0..10 {
        println!("i: {}", i);
        if i == 5 {
            break;
        }
    }

    println!("\n================================================\n");

    let mut i = 1;
    let found = false;

    while !found {
        println!("i: {}", i);
        if i == 5 {
            break;
        }
        i = i + 1;
    }

    println!("\n================================================");
    println!("🔍 Continue Statement");
    println!("================================================\n");

    for var in 0..5 {
        if var == 2 {
           println!("I encoutered a continue statement");
           continue;
         }
         println!("var: {}", var);
         println!("I did not encounter continue statement");
     }


     println!("\n================================================");
     println!("🔍 Nested loops");
     println!("================================================\n");

     for i in 1..3 {
        println!("Multiplication Table of: {}", i);
        for j in 1..6 {
            println!("{} * {} = {}", i, j, i * j);
        }
     }

     println!("\n================================================\n");

     'outer:for i in 1..4 {
        println!("Multiplication Table : {}", i);
        'inner:for j in 1..6 {
            if i == 2 { continue 'outer; }
            if j == 2 { continue 'inner; }
            println!("{} * {} = {}", i, j, i * j);
        }
     }

     let mut factorial = 1;
     for i in 1..=3 {
        factorial *= i;
     }

     println!("{}", factorial);
}
