fn main() {
    println!("Here we will explore the concept of ownership in rust!");
    string_type();
    copy_and_clone();

    let s = String::from("Hallo Welt"); //s comes into scope. A pointer on the stack is created and some memory is allocated from the heap
    takes_ownership(s); //Here s is used in the function and as the drop is called after the end of the function s goes out of scope here and may not be used anymore
    let i = 5; //i comes into scope. A instance of an integer is created on the stack
    makes_copy(i); //in the functon call a copy of i is put to the top of the stack and used in the function. i can still be used afterwards in this scope.

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2);
    println!("We can use the string s1={} and s2={}", s1, s3);

    let s4 = String::from("Hallo Welt!");
    let char_count = calculate_length(&s4);
    println!("The char count of {} is {}", s4, char_count);

    //Mutable references may be used in order to change a value
    //in another scope. There is one limitation! Only one mutable reference to a variable is
    //allowed in a scope.
    let mut s5 = String::from("This is a string");
    edit_string(&mut s5);
    println!(
        "After the function used the mutable reference the values is: {}",
        s5
    );

    //Now we learn about slices which are pointers to a subset of an array
    //In this case a string slice &str is a special case for String(char array)
    let s6 = String::from("This is a sentence.");
    let first_word = find_first_word(&s6);
    println!("The first word is {}", first_word);

    //But we could also slice any other array

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!(
        "The slice of the i32 array contains {} and {}",
        slice[0], slice[1]
    );
}

fn string_type() {
    //Here we created a string which will allocate memory from the heap and
    //can change in its size. Strings are mutable, but only if we mark them with the mut keyword
    let mut s = String::from("Hello"); //Here memory on the heap is allocated at runtime
    s.push_str(", world!");
    println!("{}", s);
} //Here the memory allocated for s is freed as s is no longer in scope

fn copy_and_clone() {
    let s1 = String::from("Test String"); //Here the string is created at the head and a pointer with the length and capacity if the string is stored in the stack.
    let s2 = s1; //In other programming languages this would be a shallow copy (only copy the pointer and add it to the stack) in rust however it is a move as the pointer to s1 becomes invalid
                 //s1 may no longer be used!
    let s3 = s2.clone(); //This is a deep copy and we now have two pointers and two allocated section of memory in the heap
    println!("s1 is no longer valid, s2={} and s3={} are clones", s2, s3);

    let i = 5;
    let j = i; //In this case i and j are both valid as an interger has a known fixed size and is therefore stored in stack.
    println!(
        "We may use both variables i={} and j={} which come from the stack.",
        i, j
    )
}

fn takes_ownership(some_string: String) {
    //some_string comes into scope
    println!("{}", some_string);
} //some_string goes out of scope and drop is called. The string may no longer be used

fn makes_copy(some_integer: i32) {
    //some:integer comes into scope
    println!("{}", some_integer)
} //Some_integer comes out of scope. some_integer might be removed from the stack here. But as it is only a copy we do not have to care about that

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

//Calculates the length of the string witghout taking the ownership of it
//the & indicates that only a reference is used
fn calculate_length(s: &String) -> usize {
    s.len()
}

//A reference parameter may also be mutable. In this case you might change it
//in the function body
fn edit_string(s: &mut String) {
    s.push_str("- Suffix");
}

//It returns the slice of the string with contains the first word
fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let bytes = bytes.iter();
    let bytes = bytes.enumerate();

    for (i, &item) in bytes {
        if item == b' ' {
            return &s[..i]; //This is the slice from the input argument containing only the first word. &s[0..i] works as well.
        }
    }

    &s[..]
}
