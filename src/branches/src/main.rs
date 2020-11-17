fn main() {
    let number = 3;
    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    multi_condition_if_else();
    if_is_an_expression();
}

fn multi_condition_if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisble by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisble by 2");
    } else {
        println!("number is neither divisble by 4, 3 nor 2");
    }
}

fn if_is_an_expression() {
    //As an expression it might be used on the right side of a let statement
    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("The value of x is {}", x);
}
