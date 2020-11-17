fn main() {
    another_function(15);
    multi_parameter_function(15, 32);
    let z = statements_and_expressions();
    println!("The statements_and_expressions function return {}", z)
}

fn another_function(x: i32) {
    println!(
        "This is another function with the parameter x: i32 that was called with the argument {}",
        x
    );
}

fn multi_parameter_function(x: i32, y: i64) {
    println!(
        "This is multi parameter function called with the arguments {} and {}",
        x, y
    );
}

fn statements_and_expressions() -> i32 {
    //An statement is any line which does not return a value
    //e.g let is a statement
    let x = 5;
    //because we may not use
    // let y = (let x = 5)
    //An expression however can be assigend to a variable
    let y = 15 + 7;
    //15 + 7 is an expression
    println!("Define a variable is a statement as seen in x with the value {}. Doing a math operation is a expression which may be part of a statement as seen in y with the value {}", x, y);

    //An expression itself does not get a ; at the end of a line as long as it is not
    //part of a statement. This is important to do function returns:
    y + 17
}
