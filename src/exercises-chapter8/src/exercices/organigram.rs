use regex::Regex;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Organisation {
    departments: HashMap<String, Vec<String>>,
}

impl Organisation {
    fn add(&mut self, name: &str, department: &str) {
        let employees = self
            .departments
            .entry(department.to_string())
            .or_insert(vec![]);
        employees.push(name.to_string());
    }
}

pub fn start() {
    println!("Starting the organigram exercise:");
    println!(
        "Please enter your company organigram with the syntax: Add \"<name>\" to \"<department name>\""
    );
    println!("As soon as your done enter: end");

    let end_condition = "end".to_string();
    let input_regex = Regex::new("Add \"(.*)\" to \"(.*)\"").expect("Invalid regular expression");
    let mut orga = Organisation {
        departments: HashMap::new(),
    };
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input"); // The programm will panic here and give us the error message defined here

        let input = input.trim();
        if input == end_condition {
            break;
        } else if input_regex.is_match(input) {
            for cap in input_regex.captures_iter(input) {
                orga.add(&cap[1], &cap[2]);
            }
        } else {
            println!("Invlaid input")
        }
    }

    println!("The organisation structure is {:?}", orga);
}
