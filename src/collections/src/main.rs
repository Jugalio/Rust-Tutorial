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

    s.push_str("Hallo Welt");

    let s1 = s1 + &s2;
    println!("{}", s1);
    let output = format!("{} {} {}", s, s1, s2);
    println!("{}", output);

    //If a substring is need we actually need to call the char method first
    let start = 5;
    let length = 5;
    println!("{:?}", get_substring(&s1, start, length));
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
