# Functions

A function is a block of code that can be reused. It is used to perform some specific task.

The functions that are customized and are written by the programmer to perform the specified tasks.

The general syntax is:
```rust
fn function_name() {
    // code here
}
```
    Naming Convention: 
    The convention for written a function name is in a snake case, i.e.,
    * All lettters should be lower case.
    * All words should be seperated by underscores.

## Call a function

The function executes when it is invoked.

The general syntax for invoking a function:
```rust
fn main(){
    function_name();
}
```

## Funciton with Parameters

Variable or values that go in the function definition are parameters
```rust
fn function_name(param1, param2, ..., paramN) {
    // statement;
}
```

Variables or values that go in their place in the functino invocation are known as arguments.
```rust
function_name (param1, param2, ..., paramN);
```

## Types of Arguments

Arguments can be passed to a function in two different ways:
* Pass by value
* Pass by reference

### Arguments Pass by Value
The values from the calling function are copied to the parameters in the function at the time the function is called. The called function can change the values of the parameter variables all it wants. This change will not be reflected in the variables passed as arguments in the calling function.

The general syntax of passing arguments by value is:

```rust
fn function_name(param1: datatype, ..., paramN:datatype) {
    // statement
}
```

### Arguments Pass by Reference
When we want the called function to make changes to the parameters such that the changes are seen by the calling functin when the call returns. The mechanism to do this is called pass arguments by reference.

The general syntax of passing arguments by reference by reference is:

```rust
fn function_name(param1:&mut datatype, ..., paramN: &mut datatype){
    //statement;
}
```

Example: 

```rust
 fn square(n:&mut i32){
  *n = *n * *n;
  println!("The value of n inside function : {}", n);
}  
fn main() {
  let  mut n = 4;
  println!("The value of n before function call : {}", n);
  println!("Invoke Function");
  square(&mut n);
  println!("The value of n after function call : {}", n);
}
```

Explanation

The function square() which takes a mutable reference (&mut) to the parameter n of type i32, the square of the variable n is calculated. Since n is a reference to a variable, to access the referenced variable’s value, a de-referencing is required. That is achieved with the *n. On the right handside, the value referenced by n is accessed and multiplied with itself. The assignment is also to *n, which means the calculated result is stored in the variable that n is referencing.

The function square() is invoked.The argument to this function is & mut n. Here, & indicates that it is a reference to the variable n and mut indicates that n can be changed inside the function square().
After the function call, the value of the n is printed.
Note: The value of n is changed within the function.

Note: The argument, as well as the parameter, is set as a mutable reference when the value is passed by reference. If the value is to be updated it is dereferenced first and then the update operation is performed.