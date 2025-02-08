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

    let length = 4;
    let width = 3;
    let (area, perimeter) = calculate_area_perimeter(length, width);
    println!("Area: {}, Perimeter: {}", area, perimeter);
    println!("================================================\n");

    let arr = [1, 2, 3, 4, 5];
    modify_my_array(arr);
    println!("Array in Driver Function: {:?}", arr);
    println!("================================================\n");

    let mut arr = [1, 2, 3, 4, 5];
    modify_my_array_2(&mut arr);
    println!("Array in Driver Function : {:?}", arr);
    println!("================================================\n");

    let n = 4;
    let fact = factorial(n);
    // print the factorial
    println!("factorial({}): {}", n, fact);
    println!("================================================\n");

    let n = 4;
    let fibo = fibonacci(n);
    println!("fibonacci({}): {}", n, fibo);
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

fn calculate_area_perimeter(x: i32, y: i32) -> (i32, i32) {
    // calculate the area and perimeter of rectangle
    let area = x * y;
    let perimeter = 2 * (x + y);

    // return the area and perimeter of rectangle
    (area, perimeter)
}

fn modify_my_array(mut arr:[i32;5]){
    arr[2] = 8;
    arr[3] = 9;

    println!("Array is my Function: {:?}", arr);
}

fn modify_my_array_2(arr:&mut [i32;5]){
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function : {:?}", arr);
}

fn factorial(n: i64) -> i64 {
    if n == 0 { // base case
        1
    }
    else {
        n * factorial(n - 1) // recursive case
    }
}

fn fibonacci(term: i64) -> i64 {
    match term {
        0 => 0,
        1 => 1,
        _ => fibonacci(term-1) + fibonacci(term-2)
    }
}