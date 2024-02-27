// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_wait_list();
    //
    // Relative path
    front_of_house::hosting::add_to_wait_list();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // let order1 = back_of_house::Appetizer::Soup;
    // let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
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
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Appetizer {
        pub fn choice(choice: &Appetizer) {
            match choice {
                Appetizer::Salad => println!("Salad!"),
                Appetizer::Soup => println!("Soup!"),
                other=> panic!("Error type"),
            }
        }
    }
}

// Bringing Paths into Scope with the use Keyword
mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_wait_list();
    }
}