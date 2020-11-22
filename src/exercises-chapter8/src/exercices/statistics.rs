use std::{collections::HashMap, io};

pub fn start() {
    println!("Starting the statistics exercise:");
    println!("Input integer values for which you would like to get statistics. Complete your input with: end");

    let mut values: Vec<i32> = Vec::new();
    let end_condition = "end".to_string();

    loop {
        let mut next_input = String::new();
        io::stdin()
            .read_line(&mut next_input)
            .expect("Failed to read user input");

        if next_input.trim() == end_condition {
            break;
        } else {
            let next_value: i32 = match next_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Failed to parse the input {} to an int32", next_input);
                    continue;
                }
            };
            values.push(next_value);
        }
    }

    if values.iter().count() == 0 {
        println!("No values added. No statistics provided")
    } else {
        println!("The median is {:?}", median(&mut 
            values));
        println!("The mode is {:?}", mode(&values));
        println!("The mean is {:?}", average(&values));
    }

    fn average(numbers: &Vec<i32>) -> f32 {
        numbers.iter().sum::<i32>() as f32 / numbers.iter().len() as f32
    }

    fn median(numbers: &mut Vec<i32>) -> Option<f32> {
        numbers.sort();
        let value_count = numbers.iter().count();
        let is_even = value_count % 2 == 0;
        if is_even {
            let index = (value_count / 2) - 1;
            match (numbers.get(index), numbers.get(index + 1)) {
                (Some(a), Some(b)) => Some(((a + b) as f32) / 2.0),
                _ => None,
            }
        } else {
            let index = ((value_count + 1) / 2) - 1;
            match numbers.get(index) {
                Some(a) => Some(*a as f32),
                None => None,
            }
        }
    }

    fn mode(numbers: &Vec<i32>) -> Option<i32> {
        let mut mode_map: HashMap<i32, i32> = HashMap::new();
        for value in numbers.iter() {
            let value_count = mode_map.entry(*value).or_insert(0);
            *value_count += 1;
        }
        mode_map
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| *k)
    }
}
