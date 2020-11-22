use std::collections::HashMap;
use std::process::Command;

fn main() {
    let mut v: Vec<i32> = Vec::new(); //Using the full syntax to create a vector
    let mut v1 = vec![14, 17, 33]; //Using a macro and let the compiler decide which type is needed

    v.push(17);
    v1.push(20);
    v1.push(38);

    let a = &v[0]; //Get the value and crash if it is not there
    let b = v.get(2); //Get an option of the value

    //Iterate over all elements in order to use them
    for i in &v {
        println!("{}", i);
    }

    //Iterate over all elements and change them (C# .Select)
    for i in &mut v {
        *i += 50; //i is just the reference to the value i here (Pointer). In order to change the value we have to follow the pointer using the dereference operator *
    }

    //Works as well
    for i in v.iter_mut() {
        *i += 1;
    }

    //This is really great as it allows us to define something similar to an interface or an absolute class even for
    //types which are not controlled by us. I love it
    let v2: Vec<SpreadsheetCell> = vec![];

    //A string is a collection of bytes. Infact it is Vec<u8>
    //There are multiple methods to create a string
    let mut s = String::new(); //An empty string
    let s1 = String::from("Hallo"); //The way we know
    let s2 = "Hallo".to_string(); //The way I will most likely use, as it is just a question of style

    s.push_str("Hällo Welt");

    let s1 = s1 + &s2;
    println!("{}", s1);
    let output = format!("{} {} {}", s, s1, s2);
    println!("{}", output);

    //If a substring is need we actually need to call the char method first
    let start = 0;
    let length = 5;
    println!("{:?}", get_substring(&s, start, length));

    let blue_team = "Blue".to_string();
    let red_team = "Red".to_string();
    let yellow_team = "Yellow".to_string();

    //We now look at hash maps
    let mut scores = HashMap::new();
    scores.insert(&blue_team, 10);
    scores.insert(&red_team, 50);

    //The same may be done when zipping two collections
    let teams = vec![&blue_team, &red_team];
    let initial_scores = vec![10, 50];
    let mut scores1: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //The next line would cause a compiler error as the values from the teams vectore are now owned by the scores1 hash map
    // teams.get(0);

    let blue_score = scores1.get(&blue_team);
    println!("The scoreboard after the first round:");
    for values in &scores {
        println!("Team {} has {} points", values.0, values.1);
    }
    println!("");

    scores.insert(&blue_team, 75); //This will per default override the Blue score
    scores.entry(&blue_team).or_insert(80); //This checks, if there already is a value for the blue team and only add 80 if no value exists. It would do nothing here
    scores.entry(&yellow_team).or_insert(80); //This however would insert a new kex-value pair

    //We may use the same syntax in order to update values
    let red_score = scores.entry(&red_team).or_insert(0); //We try get the value for the red team or enter 0, if there is no entry for this team
    *red_score += 45; //Using the dereference operator here will bring us to the value. Hence we may update it

    println!("The scoreboard after the second round:");
    for values in &scores {
        println!("Team {} has {} points", values.0, values.1);
    }
    println!("");
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Test(String),
}

fn get_substring(input: &str, start: u32, mut length: u32) -> Option<String> {
    let mut output = String::new();
    let mut i: u32 = 0;
    for c in input.chars() {
        if i >= start && length != 0 {
            output.push(c);
            length -= 1;
        } else {
            i += 1;
        }
    }
    if length > 0 || i < start {
        return None;
    } else {
        return Some(output);
    }
}
