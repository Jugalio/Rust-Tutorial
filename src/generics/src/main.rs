use core::fmt::Display;
use std::ops;

pub trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String {
        format!("{}: No summary!", self.author()) //This is a default implementation which might be overwritten by structs using this trait
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//Here the trait is implemented
impl Summary for NewsArticle {
    fn author(&self) -> String {
        self.author.to_string()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//Here the trait is implemented
impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct OpinionArticle {
    pub text: String,
}

//Here the default impl of the trait is used
impl Summary for OpinionArticle {
    fn author(&self) -> String {
        "unknown".to_string()
    }
}

#[derive(Debug)]
pub struct SomeText<'a> {
    text: &'a str,
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Copy,
{
    fn square_distance_to(&self, point: &Point<T>) -> T {
        let dist_x = point.x - self.x;
        let dist_y = point.y - self.y;
        dist_x * dist_x + dist_y * dist_y
    }
}

impl Point<f64> {
    fn euclid_distance_f64(&self, point: &Point<f64>) -> f64 {
        self.square_distance_to(point).sqrt()
    }
}

fn main() {
    let list1 = [5, 12, 100, 67, 102, 97];
    let list2 = [5, 19, 197, 67, 102, 97];

    println!("The largest numer in list 1 is {}", largest(&list1));
    println!("The largest numer in list 2 is {}", largest(&list2));

    let p1 = Point { x: 12.0, y: 17.0 };
    let p2 = Point { x: 17.8, y: 19.0 };

    println!(
        "The distance from {:?} to {:?} is {}",
        p1,
        p2,
        p1.euclid_distance_f64(&p2)
    );

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet = Tweet {
        username: "Tim".to_string(),
        content: "Haha".to_string(),
        reply: true,
        retweet: false,
    };

    let opinion_article = OpinionArticle {
        text: "who cares?".to_string(),
    };

    notify(&news_article);
    notify(&tweet);
    notify(&opinion_article);

    let a = "Hallo Welt!".to_string();
    let b = "Hello World!".to_string();
    println!("The longer statement is: {}", longest(&a, &b));
    println!(
        "The longer statement is: {}",
        longest_with_an_announcement(&a, &b, 5)
    ); // 5 is per default i32 and i32 implements Display. Therefore i32 is allowed as T

    let c = SomeText { text: &a };
    println!("This is a struct with an annotated lifetime: {:?}", c)
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn notify(item: &impl Summary) {
    println!(
        "There is a new element you might like to read. {}",
        item.summarize()
    );
}

//Neither we nor the compiler knows if it has to check the lifetime of x or y
//after calling this function. Hence it cannot validate how long the output of this function
//may be used
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
