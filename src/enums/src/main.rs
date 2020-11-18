use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

//A simple enum with some variants
#[derive(Debug)]
enum IPAdress {
    V4(String),
    V6(String),
}

//A enum can have any number of values with different variants
//This is useful to define function on it.
#[derive(Debug)]
enum Message {
    Quit,                       //No variant
    Move { x: i32, y: i32 },    //Anonymous struct
    Write(String),              //single string
    ChangeColor(i32, i32, i32), //three values of i32
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum CoinDesign {
    A,
    B,
    C,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CoinDesign),
}

struct CoinCounter {
    value: i128,
}

impl CoinCounter {
    fn insert_coin(&mut self, coin: Coin) {
        self.insert_coin_ref(&coin);
    }

    fn insert_coin_ref(&mut self, coin: &Coin) {
        self.value += match coin {
            Coin::Penny => {
                println!("Found a lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(design) => {
                println!("The quarter has the design {:?}", design);
                25
            }
        };
    }
}

fn main() {
    let home = IPAdress::V4(String::from("127.0.0.1"));
    let loopback = IPAdress::V6(String::from("::1"));

    println!(
        "The home ip adress is {:?} and the loopback is {:?}",
        home, loopback
    );

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!(
        "There are ip adress enum varaints in the standard lib already. E.g. {} and {}",
        localhost_v4, localhost_v6
    );

    let write_message = Message::Write(String::from("Hello World"));
    let move_message = Message::Move { x: 15, y: 20 };
    let quit_message = Message::Quit;
    let change_color = Message::ChangeColor(15, 22, 76);
    write_message.call();
    move_message.call();
    quit_message.call();
    change_color.call();

    //The match expression
    let mut coin_counter = CoinCounter { value: 0 };
    coin_counter.insert_coin(Coin::Nickel);
    coin_counter.insert_coin(Coin::Dime);
    coin_counter.insert_coin(Coin::Quarter(CoinDesign::A));
    coin_counter.insert_coin(Coin::Quarter(CoinDesign::B));
    coin_counter.insert_coin(Coin::Quarter(CoinDesign::C));
    coin_counter.insert_coin(Coin::Penny);
    println!(
        "All coins added to the coin counter add up to a value of {} cents",
        coin_counter.value
    );

    //Using the match expression for the Option type
    let a = Some(16);
    let b: Option<i32> = None;
    println!(
        "If using the plus_one function on an Option<i32> we get either {:?} or {:?}",
        plus_one(a),
        plus_one(b)
    );
    print_16_only(a);
    print_16_only(b);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(y) => Some(y + 1),
    }
}

fn print_16_only(value: Option<i32>) {
    if let Some(16) = value {
        println!("Sixteen");
    }
}
