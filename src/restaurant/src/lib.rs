pub use back_of_house::Breakfast; //With pub use other scopes importing this scope get acces to Breakfast
use std::fmt::Result;
use std::io::Result as IoResult; //Use as if the name is already used
use std::{cmp::Ordering, io}; //Multiple uses from the same crate

mod front_of_house;
mod back_of_house;

pub fn eat_at_restaurant() {
    //An breakfast which might be added to an order
    let meal = Breakfast::summer("Rye");
}

pub fn server_order() {}
