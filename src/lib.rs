mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // F\unction implementation
        }
        fn seat_at_table() {
            // Function implementation
        }
    }

    mod serving {
        fn take_order() {
            // Function implementation
        }
        fn serve_order() {
            // Function implementation
        }

        fn take_payment() {
            // Function implementation
        }
    }
}

pub fn eat_at_restaurant() {


 let order1 = back_of_house::Appetizer::Soup;
 let order2 = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //Absoult path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}
mod back_of_house {

    pub enum Appetizer {
 Soup,
 Salad,
 }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }
}


use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result <()>
{
    // Function implementation
    Ok(())
 
}
fn function2() -> IoResult<()> {
 
}