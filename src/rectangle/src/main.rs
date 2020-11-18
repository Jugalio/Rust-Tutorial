//Adding the derive(Debug) trait to a class let us use the debug println macro with {:?}
//This will print the instance of the struct with all its values
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Assosiated function
impl Rectangle {
    fn build(length1: u32, length2: u32) -> Rectangle {
        if length1 > length2 {
            Rectangle {
                width: length1,
                height: length2,
            }
        } else {
            Rectangle {
                width: length2,
                height: length1,
            }
        }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//Methods
impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height > rectangle.height && self.width > rectangle.width
    }
}

fn main() {
    let rectangle1 = Rectangle::build(30, 50);

    let compare_rectangles = [Rectangle::build(10, 40), Rectangle::square(35)];

    println!(
        "The area of the rectangle {:?} is {}",
        rectangle1,
        rectangle1.get_area()
    );

    for rectangle in compare_rectangles.iter() {
        if rectangle1.can_hold(rectangle) {
            println!(
                "It can hold a rectangle {:?} with an area of {}",
                rectangle,
                rectangle.get_area()
            );
        } else {
            println!(
                "It cannot hold a rectangle {:?} with an area of {}",
                rectangle,
                rectangle.get_area()
            );
        }
    }
}
