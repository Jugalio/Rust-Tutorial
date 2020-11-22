use std::io;

enum PigLationType {
    Consonant(String),
    Vowel,
    Invalid,
}

pub fn start() {
    println!("Starting the pig_latin exercise:");
    println!("Enter the word which you would like to get translated:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read the user input");

    match check_translation_option(&input.trim()) {
        PigLationType::Invalid => println!("Invalid input. No translation available"),
        PigLationType::Vowel => println!("{}-hay", input.trim()),
        PigLationType::Consonant(s) => println!("{}ay", s),
    }
}

fn check_translation_option(input: &str) -> PigLationType {
    let test = get_first_char(&input);
    println!("{:?}", test);
    match get_first_char(&input) {
        Some((c, rest)) => match c.is_numeric() {
            true => PigLationType::Invalid,
            false => match c {
                'a' | 'e' | 'o' | 'u' | 'i' | 'A' | 'E' | 'I' | 'O' | 'U' => PigLationType::Vowel,
                _ => PigLationType::Consonant(format!("{}-{}", rest, c)),
            },
        },
        None => PigLationType::Invalid,
    }
}

fn get_first_char(input: &str) -> Option<(char, &str)> {
    match input.chars().next() {
        Some(c) => Some(input.split_at(c.len_utf8())).map(|a| (a.0.chars().next().unwrap(), a.1)),
        None => None,
    }
}
