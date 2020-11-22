use std::io;

pub mod organigram;
pub mod pig_latin;
pub mod statistics;

pub fn select_exercice() {
    println!("Select which exercice you would like to run:");
    println!("1 - Statistics");
    println!("2 - Pig Latin");
    println!("3 - Organigram");

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection: u8 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("None numeric input compatible to u8.");
            0
        }
    };

    match selection {
        1 => statistics::start(),
        2 => pig_latin::start(),
        3 => organigram::start(),
        _ => println!("Invalid input. There is no exercice for {}", selection),
    };
}
