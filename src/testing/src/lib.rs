#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.can_hold_width(other) && self.height > other.height
    }

    fn can_hold_width(&self, other: &Rectangle) -> bool {
        self.width > other.width
    }
}

pub fn add_two(input: i32) -> i32 {
    input + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_larger() -> Rectangle {
        Rectangle {
            width: 8,
            height: 7,
        }
    }

    fn get_smaller() -> Rectangle {
        Rectangle {
            width: 5,
            height: 1,
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        assert!(get_larger().can_hold(&get_smaller()));
    }

    //Rust allows us to test private functions! Great thnig :)
    #[test]
    fn larger_can_hold_smaller_width() {
        assert!(get_larger().can_hold_width(&get_smaller()));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        assert!(!get_smaller().can_hold(&get_larger()));
    }

    #[test]
    fn it_works() {
        assert_eq!(17, add_two(15));
    }

    #[test]
    fn it_does_not_work() {
        assert_ne!(18, add_two(15));
    }
}
