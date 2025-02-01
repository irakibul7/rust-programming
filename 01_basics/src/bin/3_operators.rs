fn main() {
    println!("🦀 Welcome to Rust Operators! 🦀");
    println!("=================================\n");

    // SECTION 1: Arithmetic Operators
    println!("\n--- Arithmetic Operators ---");

    let a = 10;
    let b = 20;

    println!("Operand 1: {}, Operand 2: {}", a, b);
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);
    println!("Remainder: {}", a % b);

    // SECTION 2: Assignment Operators
    println!("\n--- Logical Operators ---");

    let x = true;
    let y = false;

    println!("Operand 1: {}, Operand 2: {}", x, y);
    println!("AND: {}", x && y);
    println!("OR: {}", x || y);
    println!("NOT: {}", !x);

    // SECTION 3: Comparison Operators
    println!("\n--- Comparison Operators ---");

    let a = 10;
    let b = 20;

    println!("Operand 1: {}, Operand 2: {}", a, b);
    println!("Equal: {}", a == b);
    println!("Not Equal: {}", a != b);
    println!("Greater Than: {}", a > b);
    println!("Less Than: {}", a < b);
    println!("Greater Than or Equal To: {}", a >= b);
    println!("Less Than or Equal To: {}", a <= b);

    // SECTION 4: Bitwise Operators
    println!("\n--- Bitwise Operators ---");

    let a = 10;
    let b = 20;

    println!("Operand 1: {}, Operand 2: {}", a, b);
    println!("AND: {}", a & b);
    println!("OR: {}", a | b);
    println!("XOR: {}", a ^ b);
    println!("NOT: {}", !a);
    println!("Left Shift: {}", a << 2);
    println!("Right Shift: {}", a >> 1);

    // 10 (0000 1010)
    // 20 (0001 0100)
    // ----------------
    // &   0000 0000  => 0
    // |   0001 1110  => 30
    // ^   0001 1110  => 30
    // 10 << 2  0010 1000  => 40
    // 10 >> 1  0000 0101  => 5

    // SECTION 5: Compound Assignment Operators
    println!("\n--- Compound Assignment Operators ---");

    let mut a = 10;
    a += 5;
    println!("a+=5: {}", a);

    a -= 5;
    println!("a-=5: {}", a);

    a *= 5;
    println!("a*=5: {}", a);

    a /= 5;
    println!("a/=5: {}", a);

    a %= 5;
    println!("a%=5: {}", a);

    // SECTION 6: Type Casting Operators
    println!("\n--- Type Casting Operators ---");

    let a = 10;
    let b = (a as f64) + 20.0;

    println!("a: {}, b: {}", a, b);

    // SECTION 7: Borrowing Operator
    println!("\n--- Borrowing Operator ---");

    let x = 10;
    let mut y = 20;

    // immutable reference to a variable
    let a = &x;
    println!("Value of x: {}", x);
    println!("Value of a: {}", a);

    // mutable reference to a variable
    let b = &mut y;
    println!("Value of b: {}", b);

    // dereference the mutable reference
    *b = 30;
    println!("Value of b: {}", b);
    println!("Value of y: {}", y);

}
