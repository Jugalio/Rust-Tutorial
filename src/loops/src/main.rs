fn main() {
    the_infinite_loop();
    the_while_loop();
    the_for_loop();
}

fn the_infinite_loop() {
    //A loop which stops at a condition inside
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Again");
        if counter == 10 {
            break;
        }
    }

    counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!(
        "A loop is an expression and might be used in a statement. This loop returned {}",
        result
    );
}

fn the_while_loop() {
    //Looping as long as a condition holds true might be written with a short syntax
    let mut counter = 20;
    while counter > 7 {
        counter -= 1;
    }
    println!(
        "After the while loop the counter has a value of {}",
        counter
    );
}

fn the_for_loop() {
    let a = [20, 50, 73, 89, 56];
    for element in a.iter() {
        println!("The value is {}", element);
    }

    //Iteration over a range
    for element in (1..4).rev() {
        println!("{}!", element);
    }

    println!("LIFTOFF!!!");
}
