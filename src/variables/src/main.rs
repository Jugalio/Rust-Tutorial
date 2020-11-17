const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("We have a const value of {}", MAX_POINTS);

    //Use a mutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    //Shadowing an immutable variable
    let y = 5;
    println!("The value of y is {}", y);
    let y = 6;
    println!("The value of y is {}", y);

    //Its even allowed to change the type of a variable when shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);

    //Math operations
    // addition
    let sum = 5 + 10;
    println!("The sum of 5 and 10 is {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference of 95.5 and 4.3 is {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("The product of 4 and 30 is {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient of 56.7 and 32.2 is {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("The remainder of 43 and 5 is {}", remainder);

    let t = true;
    let f: bool = false;
    println!("There is a boolean type with the values {} and {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!(
        "One may use the char type to sav values like {}, {} or {}",
        c, z, heart_eyed_cat
    );

    println!("In rust you have the choice between two compound types");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        "The first one is a tupek which may look like ({},{},{})",
        tup.0, tup.1, tup.2
    );

    //An array has a fixed size and may not change this size in rust
    //It is allocated on the stack
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr1 = [0; 5]; //inits an array with the default value 0 on all its positions
    println!("The second one is an array, which may look like [{}, {}, {}]", arr[0], arr[1], arr[2]);
    println!("It is possible to init an array of a given size with a default value. As we did for arr1 with the value {}", arr1[0]);
}